use crate::models::School;

/// A school entry for bar charts (school_id, display name, numeric value).
#[derive(Clone, Debug)]
pub struct SchoolEntry {
    pub school_id: String,
    pub name: String,
    pub value: f64,
}

/// A school entry for hype/quality analysis with both abitur and demand values.
#[derive(Clone, Debug)]
pub struct HypeEntry {
    pub school_id: String,
    pub name: String,
    pub abitur: f64,
    pub demand: f64,
}

// ── Academic Excellence ──────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct AcademicData {
    pub top_schools: Vec<SchoolEntry>,
    pub best_avg: f64,
    pub median_avg: f64,
    pub schools_rated: usize,
    pub total_schools: usize,
    pub pass_rate_95_plus: usize,
    pub pass_rate_90_95: usize,
    pub pass_rate_below_90: usize,
    pub pct_above_95: f64,
}

pub fn compute_academic(schools: &[School]) -> AcademicData {
    let mut with_abitur: Vec<(String, String, f64)> = schools
        .iter()
        .filter_map(|s| {
            s.abitur_average
                .map(|avg| (s.school_id.clone(), s.name.clone(), avg))
        })
        .collect();
    with_abitur.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

    let schools_rated = with_abitur.len();
    let best_avg = with_abitur.first().map(|s| s.2).unwrap_or(0.0);
    let median_avg = if schools_rated > 0 {
        with_abitur[schools_rated / 2].2
    } else {
        0.0
    };

    let top_schools = with_abitur
        .into_iter()
        .take(10)
        .map(|(id, name, val)| SchoolEntry {
            school_id: id,
            name,
            value: val,
        })
        .collect();

    let with_pass_rate: Vec<f64> = schools.iter().filter_map(|s| s.abitur_pass_rate).collect();
    let pass_rate_95_plus = with_pass_rate.iter().filter(|&&r| r >= 95.0).count();
    let pass_rate_90_95 = with_pass_rate.iter().filter(|&&r| r >= 90.0 && r < 95.0).count();
    let pass_rate_below_90 = with_pass_rate.iter().filter(|&&r| r < 90.0).count();
    let pct_above_95 = if with_pass_rate.is_empty() { 0.0 } else { pass_rate_95_plus as f64 / with_pass_rate.len() as f64 * 100.0 };

    AcademicData {
        top_schools,
        best_avg,
        median_avg,
        schools_rated,
        total_schools: schools.len(),
        pass_rate_95_plus,
        pass_rate_90_95,
        pass_rate_below_90,
        pct_above_95,
    }
}

// ── Supply vs Demand ─────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct DistrictCount {
    pub name: String,
    pub count: usize,
}

#[derive(Clone, Debug)]
pub struct SupplyDemandData {
    pub total_schools: usize,
    pub accept_4th: usize,
    pub avg_student_teacher: f64,
    pub oversubscribed: Vec<SchoolEntry>,
    pub districts: Vec<DistrictCount>,
}

pub fn compute_supply_demand(schools: &[School]) -> SupplyDemandData {
    let total_schools = schools.len();
    let accept_4th = schools
        .iter()
        .filter(|s| s.accepts_after_4th_grade == Some(true))
        .count();

    let (total_students, total_teachers) = schools.iter().fold((0u32, 0u32), |(s, t), school| {
        (
            s + school.student_count.unwrap_or(0),
            t + school.teacher_count.unwrap_or(0),
        )
    });
    let avg_student_teacher = if total_teachers > 0 {
        total_students as f64 / total_teachers as f64
    } else {
        0.0
    };

    let mut with_demand: Vec<(String, String, f64)> = schools
        .iter()
        .filter_map(|s| {
            s.admission_requirements
                .as_ref()
                .and_then(|ar| ar.demand_ratio)
                .map(|r| (s.school_id.clone(), s.name.clone(), r))
        })
        .collect();
    with_demand.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let oversubscribed = with_demand
        .into_iter()
        .take(10)
        .map(|(id, name, val)| SchoolEntry {
            school_id: id,
            name,
            value: val,
        })
        .collect();

    let mut district_map = std::collections::HashMap::<String, usize>::new();
    for s in schools {
        *district_map.entry(s.district.clone()).or_default() += 1;
    }
    let mut districts: Vec<DistrictCount> = district_map
        .into_iter()
        .map(|(name, count)| DistrictCount { name, count })
        .collect();
    districts.sort_by(|a, b| b.count.cmp(&a.count));

    SupplyDemandData {
        total_schools,
        accept_4th,
        avg_student_teacher,
        oversubscribed,
        districts,
    }
}

// ── Does Hype = Quality? ────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct HypeQualityData {
    pub correlation_r: f64,
    pub hidden_gems: Vec<HypeEntry>,
    pub overhyped: Vec<HypeEntry>,
    pub best_combo: Option<SchoolEntry>,
    pub worst_combo: Option<SchoolEntry>,
    pub best_combo_demand: Option<f64>,
    pub worst_combo_demand: Option<f64>,
    pub schools_analyzed: usize,
}

pub fn compute_hype_quality(schools: &[School]) -> HypeQualityData {
    // Collect schools with both abitur and demand_ratio
    let pairs: Vec<(String, String, f64, f64)> = schools
        .iter()
        .filter_map(|s| {
            let abitur = s.abitur_average?;
            let demand = s.admission_requirements.as_ref()?.demand_ratio?;
            Some((s.school_id.clone(), s.name.clone(), abitur, demand))
        })
        .collect();

    let n = pairs.len() as f64;
    let schools_analyzed = pairs.len();

    // Pearson correlation: r = (nΣxy - ΣxΣy) / sqrt((nΣx²-(Σx)²)(nΣy²-(Σy)²))
    let (sum_x, sum_y, sum_xy, sum_x2, sum_y2) =
        pairs
            .iter()
            .fold((0.0, 0.0, 0.0, 0.0, 0.0), |(sx, sy, sxy, sx2, sy2), p| {
                let (x, y) = (p.3, p.2); // demand vs abitur
                (sx + x, sy + y, sxy + x * y, sx2 + x * x, sy2 + y * y)
            });

    let correlation_r = if n > 2.0 {
        let num = n * sum_xy - sum_x * sum_y;
        let den = ((n * sum_x2 - sum_x * sum_x) * (n * sum_y2 - sum_y * sum_y)).sqrt();
        if den > 0.0 { num / den } else { 0.0 }
    } else {
        0.0
    };

    // Hidden gems: good abitur (< median) but low demand (< 1.0)
    let median_abitur = {
        let mut abis: Vec<f64> = pairs.iter().map(|p| p.2).collect();
        abis.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if abis.is_empty() {
            2.0
        } else {
            abis[abis.len() / 2]
        }
    };

    let mut hidden_gems: Vec<HypeEntry> = pairs
        .iter()
        .filter(|p| p.2 < median_abitur && p.3 < 1.0)
        .map(|p| HypeEntry {
            school_id: p.0.clone(),
            name: p.1.clone(),
            abitur: p.2,
            demand: p.3,
        })
        .collect();
    hidden_gems.sort_by(|a, b| a.abitur.partial_cmp(&b.abitur).unwrap());
    hidden_gems.truncate(3);

    let mut overhyped: Vec<HypeEntry> = pairs
        .iter()
        .filter(|p| p.2 > median_abitur && p.3 > 1.2)
        .map(|p| HypeEntry {
            school_id: p.0.clone(),
            name: p.1.clone(),
            abitur: p.2,
            demand: p.3,
        })
        .collect();
    overhyped.sort_by(|a, b| b.demand.partial_cmp(&a.demand).unwrap());
    overhyped.truncate(3);

    // Best combo: lowest abitur among high-demand schools
    let best_combo_pair = pairs
        .iter()
        .filter(|p| p.3 > 1.2)
        .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let best_combo = best_combo_pair.map(|p| SchoolEntry {
        school_id: p.0.clone(),
        name: p.1.clone(),
        value: p.2,
    });
    let best_combo_demand = best_combo_pair.map(|p| p.3);

    // Worst combo: highest abitur among low-demand schools
    let worst_combo_pair = pairs
        .iter()
        .filter(|p| p.3 < 0.8)
        .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let worst_combo = worst_combo_pair.map(|p| SchoolEntry {
        school_id: p.0.clone(),
        name: p.1.clone(),
        value: p.2,
    });
    let worst_combo_demand = worst_combo_pair.map(|p| p.3);

    HypeQualityData {
        correlation_r,
        hidden_gems,
        overhyped,
        best_combo,
        worst_combo,
        best_combo_demand,
        worst_combo_demand,
        schools_analyzed,
    }
}

// ── Private vs Public ────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ComparisonData {
    pub left_count: usize,
    pub right_count: usize,
    pub left_label: &'static str,
    pub right_label: &'static str,
    pub metrics: Vec<ComparisonMetric>,
    pub insight: String,
}

#[derive(Clone, Debug)]
pub struct ComparisonMetric {
    pub label: String,
    pub hint: String,
    pub left_value: String,
    pub right_value: String,
    pub left_wins: bool,
    pub left_num: f64,
    pub right_num: f64,
}

pub fn compute_private_public(schools: &[School]) -> ComparisonData {
    let private: Vec<&School> = schools
        .iter()
        .filter(|s| s.traeger.as_deref() == Some("privat"))
        .collect();
    let public: Vec<&School> = schools
        .iter()
        .filter(|s| s.traeger.as_deref() != Some("privat"))
        .collect();

    let avg = |list: &[&School], f: fn(&School) -> Option<f64>| -> f64 {
        let vals: Vec<f64> = list.iter().filter_map(|s| f(s)).collect();
        if vals.is_empty() {
            0.0
        } else {
            vals.iter().sum::<f64>() / vals.len() as f64
        }
    };

    let priv_abitur = avg(&private, |s| s.abitur_average);
    let pub_abitur = avg(&public, |s| s.abitur_average);
    let priv_pass = avg(&private, |s| s.abitur_pass_rate);
    let pub_pass = avg(&public, |s| s.abitur_pass_rate);
    let priv_ratio = avg(&private, |s| {
        let sc = s.student_count? as f64;
        let tc = s.teacher_count? as f64;
        if tc > 0.0 { Some(sc / tc) } else { None }
    });
    let pub_ratio = avg(&public, |s| {
        let sc = s.student_count? as f64;
        let tc = s.teacher_count? as f64;
        if tc > 0.0 { Some(sc / tc) } else { None }
    });
    let priv_size = avg(&private, |s| s.student_count.map(|c| c as f64));
    let pub_size = avg(&public, |s| s.student_count.map(|c| c as f64));

    let metrics = vec![
        ComparisonMetric {
            label: "Abitur Average".into(),
            hint: "lower is better".into(),
            left_value: format!("{:.2}", priv_abitur),
            right_value: format!("{:.2}", pub_abitur),
            left_wins: priv_abitur < pub_abitur && priv_abitur > 0.0,
            left_num: priv_abitur,
            right_num: pub_abitur,
        },
        ComparisonMetric {
            label: "Pass Rate".into(),
            hint: "higher is better".into(),
            left_value: format!("{:.1}%", priv_pass),
            right_value: format!("{:.1}%", pub_pass),
            left_wins: priv_pass > pub_pass,
            left_num: priv_pass,
            right_num: pub_pass,
        },
        ComparisonMetric {
            label: "Student / Teacher Ratio".into(),
            hint: "lower is better".into(),
            left_value: format!("{:.1}", priv_ratio),
            right_value: format!("{:.1}", pub_ratio),
            left_wins: priv_ratio < pub_ratio && priv_ratio > 0.0,
            left_num: priv_ratio,
            right_num: pub_ratio,
        },
        ComparisonMetric {
            label: "Avg School Size".into(),
            hint: "students enrolled".into(),
            left_value: format!("{:.0}", priv_size),
            right_value: format!("{:.0}", pub_size),
            left_wins: false,
            left_num: priv_size,
            right_num: pub_size,
        },
    ];

    let gap = (pub_abitur - priv_abitur).abs();
    let insight = format!(
        "With a {:.2} grade-point gap, the performance difference is notable — private schools {} on average.",
        gap,
        if priv_abitur < pub_abitur { "outperform" } else { "underperform" }
    );

    ComparisonData {
        left_count: private.len(),
        right_count: public.len(),
        left_label: "Private",
        right_label: "Public",
        metrics,
        insight,
    }
}

// ── 4th Grade Advantage ──────────────────────────────────────────────

pub fn compute_grade4(schools: &[School]) -> ComparisonData {
    let grundstaendig: Vec<&School> = schools
        .iter()
        .filter(|s| s.accepts_after_4th_grade == Some(true))
        .collect();
    let standard: Vec<&School> = schools
        .iter()
        .filter(|s| s.accepts_after_4th_grade != Some(true))
        .collect();

    let avg = |list: &[&School], f: fn(&School) -> Option<f64>| -> f64 {
        let vals: Vec<f64> = list.iter().filter_map(|s| f(s)).collect();
        if vals.is_empty() {
            0.0
        } else {
            vals.iter().sum::<f64>() / vals.len() as f64
        }
    };

    let g_abitur = avg(&grundstaendig, |s| s.abitur_average);
    let s_abitur = avg(&standard, |s| s.abitur_average);
    let g_demand = avg(&grundstaendig, |s| {
        s.admission_requirements.as_ref()?.demand_ratio
    });
    let s_demand = avg(&standard, |s| {
        s.admission_requirements.as_ref()?.demand_ratio
    });
    let g_pass = avg(&grundstaendig, |s| s.abitur_pass_rate);
    let s_pass = avg(&standard, |s| s.abitur_pass_rate);
    let g_size = avg(&grundstaendig, |s| s.student_count.map(|c| c as f64));
    let s_size = avg(&standard, |s| s.student_count.map(|c| c as f64));

    let metrics = vec![
        ComparisonMetric {
            label: "Abitur Average".into(),
            hint: "lower is better".into(),
            left_value: format!("{:.2}", g_abitur),
            right_value: format!("{:.2}", s_abitur),
            left_wins: g_abitur < s_abitur && g_abitur > 0.0,
            left_num: g_abitur,
            right_num: s_abitur,
        },
        ComparisonMetric {
            label: "Demand Ratio".into(),
            hint: "higher = more sought-after".into(),
            left_value: format!("{:.2}", g_demand),
            right_value: format!("{:.2}", s_demand),
            left_wins: g_demand > s_demand,
            left_num: g_demand,
            right_num: s_demand,
        },
        ComparisonMetric {
            label: "Pass Rate".into(),
            hint: "higher is better".into(),
            left_value: format!("{:.1}%", g_pass),
            right_value: format!("{:.1}%", s_pass),
            left_wins: g_pass > s_pass,
            left_num: g_pass,
            right_num: s_pass,
        },
        ComparisonMetric {
            label: "Avg School Size".into(),
            hint: "students enrolled".into(),
            left_value: format!("{:.0}", g_size),
            right_value: format!("{:.0}", s_size),
            left_wins: false,
            left_num: g_size,
            right_num: s_size,
        },
    ];

    let pct = if schools.is_empty() {
        0.0
    } else {
        grundstaendig.len() as f64 / schools.len() as f64 * 100.0
    };
    let gap = (s_abitur - g_abitur).abs();
    let insight = format!(
        "{:.1}% of Berlin Gymnasien offer early entry. Schools with grundständig programs (grades 5-12) show a {:.2} grade-point advantage in Abitur results.",
        pct, gap,
    );

    ComparisonData {
        left_count: grundstaendig.len(),
        right_count: standard.len(),
        left_label: "Grundständig",
        right_label: "Standard",
        metrics,
        insight,
    }
}

// ── District Power Rankings ──────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct DistrictRow {
    pub rank: usize,
    pub name: String,
    pub avg_abitur: f64,
    pub avg_demand: f64,
    pub avg_pass: f64,
    pub total_students: usize,
}

#[derive(Clone, Debug)]
pub struct DistrictRankingData {
    pub districts: Vec<DistrictRow>,
    pub grade_gap: f64,
    pub total_students: usize,
    pub city_avg: f64,
}

pub fn compute_district_rankings(schools: &[School]) -> DistrictRankingData {
    let mut district_map: std::collections::HashMap<String, Vec<&School>> =
        std::collections::HashMap::new();
    for s in schools {
        district_map.entry(s.district.clone()).or_default().push(s);
    }

    let avg_f = |list: &[&School], f: fn(&School) -> Option<f64>| -> f64 {
        let vals: Vec<f64> = list.iter().filter_map(|s| f(s)).collect();
        if vals.is_empty() {
            0.0
        } else {
            vals.iter().sum::<f64>() / vals.len() as f64
        }
    };

    let mut rows: Vec<DistrictRow> = district_map
        .iter()
        .map(|(name, list)| {
            let avg_abitur = avg_f(list, |s| s.abitur_average);
            let avg_demand = avg_f(list, |s| s.admission_requirements.as_ref()?.demand_ratio);
            let avg_pass = avg_f(list, |s| s.abitur_pass_rate);
            let total_students: usize =
                list.iter().filter_map(|s| s.student_count).sum::<u32>() as usize;
            DistrictRow {
                rank: 0,
                name: name.clone(),
                avg_abitur,
                avg_demand,
                avg_pass,
                total_students,
            }
        })
        .collect();

    // Sort by composite score (lower abitur = better, higher demand = better, higher pass = better)
    rows.sort_by(|a, b| {
        let score_a = a.avg_abitur - a.avg_demand * 0.5 - a.avg_pass * 0.01;
        let score_b = b.avg_abitur - b.avg_demand * 0.5 - b.avg_pass * 0.01;
        score_a
            .partial_cmp(&score_b)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for (i, row) in rows.iter_mut().enumerate() {
        row.rank = i + 1;
    }

    let total_students: usize = rows.iter().map(|r| r.total_students).sum();

    let all_abitur: Vec<f64> = schools.iter().filter_map(|s| s.abitur_average).collect();
    let city_avg = if all_abitur.is_empty() {
        0.0
    } else {
        all_abitur.iter().sum::<f64>() / all_abitur.len() as f64
    };

    let grade_gap = if rows.len() >= 2 {
        let best = rows.first().map(|r| r.avg_abitur).unwrap_or(0.0);
        let worst = rows.last().map(|r| r.avg_abitur).unwrap_or(0.0);
        (worst - best).abs()
    } else {
        0.0
    };

    DistrictRankingData {
        districts: rows,
        grade_gap,
        total_students,
        city_avg,
    }
}
