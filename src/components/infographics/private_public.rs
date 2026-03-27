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
            let left_class = if metric.left_wins {
                "ig-metric-left ig-winner"
            } else {
                "ig-metric-left"
            };
            let right_class = if !metric.left_wins {
                "ig-metric-right ig-winner"
            } else {
                "ig-metric-right"
            };
            let left_value = metric.left_value.clone();
            let right_value = metric.right_value.clone();
            let label = metric.label.clone();
            let hint = metric.hint.clone();

            view! {
                <div class="ig-metric-row">
                    <span class=left_class>{left_value}</span>
                    <div class="ig-metric-info">
                        <span class="ig-metric-label">{label}</span>
                        <span class="ig-metric-hint">{hint}</span>
                    </div>
                    <span class=right_class>{right_value}</span>
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
                <div class="ig-count-vs">"vs"</div>
                <div class="ig-count">
                    <span class="ig-count-value">{right_count}</span>
                    <span class="ig-count-label">"Public"</span>
                </div>
            </div>
            <div class="ig-section">
                <h3 class="ig-section-title">"HEAD TO HEAD"</h3>
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
            <div class="ig-insight">
                <p>{insight}</p>
            </div>
        </div>
    }
}
