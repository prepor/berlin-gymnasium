use leptos::prelude::*;

use crate::hash_router::navigate_hash;
use crate::models::School;

use super::data::compute_grade4;

#[component]
pub fn Grade4Advantage(schools: Vec<School>) -> impl IntoView {
    let data = compute_grade4(&schools);

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
            <div class="ig-section">
                <h3 class="ig-section-title">"Performance Gap"</h3>
                <div class="ig-comparison">
                    {metrics_view}
                </div>
            </div>
            <div class="ig-insight">
                <h4 class="ig-insight-label">"What this means"</h4>
                <p>{insight}</p>
            </div>
            <div class="ig-cta-row">
                <a
                    class="ig-cta"
                    href="javascript:void(0)"
                    on:click=move |ev: leptos::ev::MouseEvent| {
                        ev.prevent_default();
                        navigate_hash("/?grundstaendig=ja", false);
                    }
                >
                    "View grundst\u{00E4}ndig schools \u{2192}"
                </a>
            </div>
        </div>
    }
}
