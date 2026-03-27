use leptos::prelude::*;

use crate::models::School;

use super::data::compute_private_public;

#[component]
pub fn PrivatePublic(schools: Vec<School>) -> impl IntoView {
    let data = compute_private_public(&schools);

    let left_count = data.left_count;
    let right_count = data.right_count;
    let insight = data.insight.clone();

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
                <h2 class="ig-title">"Private vs Public"</h2>
                <p class="ig-subtitle">"A tale of two systems in Berlin education"</p>
            </div>
            <div class="ig-count-row">
                <div class="ig-count">
                    <span class="ig-count-value">{left_count}</span>
                    <span class="ig-count-label">"Private"</span>
                </div>
                <div class="ig-count">
                    <span class="ig-count-value" style="color: #fff">{right_count}</span>
                    <span class="ig-count-label">"Public"</span>
                </div>
            </div>
            <div class="ig-section">
                <span class="ig-eyebrow">"HEAD TO HEAD"</span>
                <div class="ig-compare-legend">
                    <span class="ig-compare-legend-item">
                        <span class="ig-compare-legend-dot" style="background: #A855F7"></span>
                        " Private"
                    </span>
                    <span class="ig-compare-legend-item">
                        <span class="ig-compare-legend-dot" style="background: #52525B"></span>
                        " Public"
                    </span>
                </div>
                <div class="ig-comparison">
                    {metrics_view}
                </div>
            </div>
            <div class="ig-callout">
                <span class="ig-callout-icon">"i"</span>
                <p class="ig-callout-text">
                    "Private schools don\u{2019}t participate in the public lottery \u{2014} no demand_ratio data available"
                </p>
            </div>
            <p class="ig-insight-text" style="font-style: italic">{insight}</p>
        </div>
    }
}
