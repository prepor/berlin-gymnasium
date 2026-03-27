use leptos::prelude::*;

use crate::models::School;

use super::data::compute_grade4;

#[component]
pub fn Grade4Advantage(schools: Vec<School>) -> impl IntoView {
    let data = compute_grade4(&schools);

    let left_count = data.left_count;
    let right_count = data.right_count;
    let insight = data.insight.clone();
    let pct = if schools.is_empty() {
        0.0
    } else {
        left_count as f64 / (left_count + right_count) as f64 * 100.0
    };

    let metrics_view = data
        .metrics
        .iter()
        .map(|metric| {
            let max_val = metric.left_num.max(metric.right_num);
            let left_pct = if max_val > 0.0 {
                (metric.left_num / max_val * 100.0).max(5.0)
            } else {
                5.0
            };
            let right_pct = if max_val > 0.0 {
                (metric.right_num / max_val * 100.0).max(5.0)
            } else {
                5.0
            };
            let left_value = metric.left_value.clone();
            let right_value = metric.right_value.clone();
            let label = metric.label.clone();
            let hint = metric.hint.clone();

            view! {
                <div class="ig-bar-compare">
                    <div class="ig-bar-compare-label">
                        <span class="ig-bar-compare-name">{label}</span>
                        <span class="ig-bar-compare-hint">{" \u{00B7} "}{hint}</span>
                    </div>
                    <div class="ig-bar-compare-bars">
                        <div class="ig-bar-compare-row">
                            <div class="ig-bar-track">
                                <div
                                    class="ig-bar-fill"
                                    style=format!("width: {}%; background: #A855F7", left_pct)
                                ></div>
                            </div>
                            <span class="ig-bar-value">{left_value}</span>
                        </div>
                        <div class="ig-bar-compare-row">
                            <div class="ig-bar-track">
                                <div
                                    class="ig-bar-fill"
                                    style=format!("width: {}%; background: #52525B", right_pct)
                                ></div>
                            </div>
                            <span class="ig-bar-value">{right_value}</span>
                        </div>
                    </div>
                </div>
            }
        })
        .collect_view();

    view! {
        <div class="ig-card">
            <div class="ig-header">
                <span class="ig-eyebrow">"BERLIN GYMNASIEN"</span>
                <h2 class="ig-title">"The 4th Grade Advantage"</h2>
                <p class="ig-subtitle">"Schools accepting early entry consistently outperform"</p>
            </div>
            <div class="ig-count-row">
                <div class="ig-count">
                    <span class="ig-count-value">{left_count}</span>
                    <span class="ig-count-label">"Schools"</span>
                    <span class="ig-count-desc">"Accept after 4th grade"</span>
                </div>
                <div class="ig-count-vs">"vs"</div>
                <div class="ig-count">
                    <span class="ig-count-value">{right_count}</span>
                    <span class="ig-count-label">"Schools"</span>
                    <span class="ig-count-desc">"Standard entry only"</span>
                </div>
            </div>
            <div class="ig-section ig-section--flat">
                <h3 class="ig-section-title">"Performance Gap"</h3>
                <div class="ig-compare-legend">
                    <span class="ig-compare-legend-item">
                        <span class="ig-compare-legend-dot" style="background: #A855F7"></span>
                        " 4th Grade"
                    </span>
                    <span class="ig-compare-legend-item">
                        <span class="ig-compare-legend-dot" style="background: #52525B"></span>
                        " Standard"
                    </span>
                </div>
                <div class="ig-comparison">
                    {metrics_view}
                </div>
            </div>
            <div class="ig-insight">
                <h4 class="ig-insight-label">"What this means"</h4>
                <p>{insight}</p>
            </div>
            <div class="ig-big-stat">
                <span class="ig-big-stat-value">{format!("{:.1}%", pct)}</span>
                <span class="ig-big-stat-label">"of Berlin Gymnasien offer early entry"</span>
            </div>
        </div>
    }
}
