use leptos::prelude::*;

use crate::hash_router::navigate_hash;
use crate::models::School;

use super::data::compute_supply_demand;

#[component]
pub fn SupplyDemand(schools: Vec<School>) -> impl IntoView {
    let data = compute_supply_demand(&schools);

    let total_schools = data.total_schools;
    let accept_4th = data.accept_4th;
    let avg_student_teacher = data.avg_student_teacher;

    let max_demand = data
        .oversubscribed
        .iter()
        .map(|e| e.value)
        .fold(0.0_f64, f64::max);

    let oversubscribed_view = data
        .oversubscribed
        .iter()
        .map(|entry| {
            let school_id = entry.school_id.clone();
            let name = entry.name.clone();
            let value = entry.value;
            let width = if max_demand > 0.0 {
                (value / max_demand * 100.0).max(5.0).min(100.0)
            } else {
                5.0
            };

            view! {
                <div class="ig-bar-row">
                    <a
                        class="ig-link ig-bar-label"
                        href="javascript:void(0)"
                        on:click=move |ev: leptos::ev::MouseEvent| {
                            ev.prevent_default();
                            navigate_hash(&format!("/school/{}", school_id), false);
                        }
                    >
                        {name}
                    </a>
                    <div class="ig-bar-track">
                        <div
                            class="ig-bar-fill"
                            style=format!("width: {}%; background: #A855F7", width)
                        ></div>
                    </div>
                    <span class="ig-bar-value">{format!("{:.2}", value)}</span>
                </div>
            }
        })
        .collect_view();

    let max_district_count = data
        .districts
        .iter()
        .map(|d| d.count)
        .max()
        .unwrap_or(1);

    let districts_view = data
        .districts
        .iter()
        .map(|district| {
            let name = district.name.clone();
            let name_for_click = district.name.clone();
            let count = district.count;
            let width =
                (count as f64 / max_district_count as f64 * 100.0).max(5.0).min(100.0);

            view! {
                <div class="ig-bar-row">
                    <a
                        class="ig-link ig-bar-label"
                        href="javascript:void(0)"
                        on:click=move |ev: leptos::ev::MouseEvent| {
                            ev.prevent_default();
                            navigate_hash(
                                &format!("/?district={}&view=map", name_for_click),
                                false,
                            );
                        }
                    >
                        {name}
                    </a>
                    <div class="ig-bar-track">
                        <div
                            class="ig-bar-fill"
                            style=format!("width: {}%; background: #3B82F6", width)
                        ></div>
                    </div>
                    <span class="ig-bar-value">{count}</span>
                </div>
            }
        })
        .collect_view();

    view! {
        <div class="ig-card">
            <div class="ig-header">
                <span class="ig-eyebrow">"BERLIN GYMNASIEN"</span>
                <h2 class="ig-title">"Supply vs Demand"</h2>
                <p class="ig-subtitle">"Which schools are the most sought-after — and where are they?"</p>
            </div>
            <div class="ig-stats-row">
                <div class="ig-stat">
                    <span class="ig-stat-value">{total_schools}</span>
                    <span class="ig-stat-label">"Total Schools"</span>
                </div>
                <div class="ig-stat">
                    <span class="ig-stat-value">{accept_4th}</span>
                    <span class="ig-stat-label">"Accept After 4th"</span>
                </div>
                <div class="ig-stat">
                    <span class="ig-stat-value">{format!("{:.1}", avg_student_teacher)}</span>
                    <span class="ig-stat-label">"Avg Student/Teacher"</span>
                </div>
            </div>
            <div class="ig-section">
                <h3 class="ig-section-title">"Most Oversubscribed Schools"</h3>
                <p class="ig-section-hint">"Demand ratio = first choices ÷ available places"</p>
                <div class="ig-bar-chart">
                    {oversubscribed_view}
                </div>
                <p class="ig-ref-note">"▬ 1.0 = balanced  |  > 1.0 = oversubscribed"</p>
            </div>
            <div class="ig-section">
                <h3 class="ig-section-title">"Schools by District"</h3>
                <p class="ig-section-hint">"Distribution across Berlin's Bezirke"</p>
                <div class="ig-bar-chart">
                    {districts_view}
                </div>
            </div>
        </div>
    }
}
