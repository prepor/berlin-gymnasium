use leptos::prelude::*;

use crate::hash_router::navigate_hash;
use crate::models::School;

use super::data::compute_district_rankings;

#[component]
pub fn DistrictPowerRankings(schools: Vec<School>) -> impl IntoView {
    let data = compute_district_rankings(&schools);

    let grade_gap = data.grade_gap;
    let total_students = data.total_students;
    let city_avg = data.city_avg;

    let top_district_name = data
        .districts
        .first()
        .map(|d| d.name.clone())
        .unwrap_or_default();
    let last_rank = data.districts.len();
    let insight_text = format!(
        "{} leads with the best scores. A {:.2} grade-point gap separates #1 from #{}, revealing educational inequality across Berlin\u{2019}s districts.",
        top_district_name, grade_gap, last_rank,
    );

    let rows_view = data
        .districts
        .iter()
        .map(|row| {
            let rank = row.rank;
            let name = row.name.clone();
            let nav_name = row.name.clone();
            let avg_abitur = format!("{:.2}", row.avg_abitur);
            let avg_demand = format!("{:.2}", row.avg_demand);
            let avg_pass = format!("{:.1}%", row.avg_pass);
            let students_formatted = format_students(row.total_students);
            let is_bottom = rank > last_rank - 2;
            let row_class = if rank <= 3 {
                "ig-table-row ig-table-top3"
            } else if is_bottom {
                "ig-table-row ig-table-bottom"
            } else {
                "ig-table-row"
            };

            let rank_view = if rank <= 3 {
                view! { <span class="ig-td"><span class="ig-rank-badge">{rank}</span></span> }.into_any()
            } else if is_bottom {
                view! { <span class="ig-td ig-rank-bottom">{rank}</span> }.into_any()
            } else {
                view! { <span class="ig-td ig-rank">{rank}</span> }.into_any()
            };

            let abitur_class = if is_bottom { "ig-td ig-td-red" } else { "ig-td" };

            let separator = if rank == last_rank - 1 {
                Some(view! { <div class="ig-table-sep"></div> })
            } else {
                None
            };

            view! {
                {separator}
                <div class=row_class>
                    {rank_view}
                    <a
                        class="ig-link ig-td"
                        href="javascript:void(0)"
                        on:click=move |ev: leptos::ev::MouseEvent| {
                            ev.prevent_default();
                            navigate_hash(
                                &format!("/?district={}&view=map", nav_name),
                                false,
                            );
                        }
                    >
                        {name}
                    </a>
                    <span class=abitur_class>{avg_abitur}</span>
                    <span class="ig-td">{avg_demand}</span>
                    <span class="ig-td">{avg_pass}</span>
                    <span class="ig-td">{students_formatted}</span>
                </div>
            }
        })
        .collect_view();

    let students_display = format!("{}K", total_students / 1000);

    view! {
        <div class="ig-card">
            <div class="ig-header">
                <span class="ig-eyebrow">"BERLIN GYMNASIEN"</span>
                <h2 class="ig-title">"District Power Rankings"</h2>
                <p class="ig-subtitle">"Which Bezirk delivers the best education outcomes?"</p>
            </div>
            <div class="ig-table">
                <div class="ig-table-header">
                    <span class="ig-th">"#"</span>
                    <span class="ig-th">"DISTRICT"</span>
                    <span class="ig-th">"ABITUR"</span>
                    <span class="ig-th">"DEMAND"</span>
                    <span class="ig-th">"PASS %"</span>
                    <span class="ig-th">"STUDENTS"</span>
                </div>
                {rows_view}
            </div>
            <div class="ig-insight">
                <h4 class="ig-insight-label">"KEY INSIGHT"</h4>
                <p>{insight_text}</p>
            </div>
            <div class="ig-stats-row">
                <div class="ig-stat">
                    <span class="ig-stat-value">{format!("{:.2}", grade_gap)}</span>
                    <span class="ig-stat-label">"Grade Gap 1st\u{2194}Last"</span>
                </div>
                <div class="ig-stat">
                    <span class="ig-stat-value">{students_display}</span>
                    <span class="ig-stat-label">"Total Students"</span>
                </div>
                <div class="ig-stat">
                    <span class="ig-stat-value">{format!("{:.2}", city_avg)}</span>
                    <span class="ig-stat-label">"City-wide Average"</span>
                </div>
            </div>
        </div>
    }
}

/// Format student count with comma separators.
fn format_students(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    result.chars().rev().collect()
}
