use leptos::prelude::*;

use crate::hash_router::navigate_hash;
use crate::models::School;

use super::data::compute_academic;

#[component]
pub fn AcademicExcellence(schools: Vec<School>) -> impl IntoView {
    let data = compute_academic(&schools);

    let top_schools_view = data
        .top_schools
        .iter()
        .map(|entry| {
            let school_id = entry.school_id.clone();
            let name = entry.name.clone();
            let value = entry.value;
            let width = ((3.0 - value) / (3.0 - 1.0) * 100.0).max(5.0).min(100.0);

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

    let best_avg = data.best_avg;
    let median_avg = data.median_avg;
    let schools_rated = data.schools_rated;
    let total_schools = data.total_schools;

    view! {
        <div class="ig-card">
            <div class="ig-header">
                <span class="ig-eyebrow">"BERLIN GYMNASIEN"</span>
                <h2 class="ig-title">"Academic Excellence"</h2>
                <p class="ig-subtitle">"Top Abitur Performance Across Berlin Gymnasien"</p>
            </div>
            <div class="ig-section">
                <h3 class="ig-section-title">"Top 10 Abitur Averages"</h3>
                <p class="ig-section-hint">"Lower score = stronger performance (German grading scale)"</p>
                <div class="ig-bar-chart">
                    {top_schools_view}
                </div>
            </div>
            <div class="ig-stats-row">
                <div class="ig-stat">
                    <span class="ig-stat-value">{format!("{:.2}", best_avg)}</span>
                    <span class="ig-stat-label">"Best Average"</span>
                </div>
                <div class="ig-stat">
                    <span class="ig-stat-value">{format!("~{:.2}", median_avg)}</span>
                    <span class="ig-stat-label">"Median Average"</span>
                </div>
                <div class="ig-stat">
                    <span class="ig-stat-value">{format!("{}/{}", schools_rated, total_schools)}</span>
                    <span class="ig-stat-label">"Schools Rated"</span>
                </div>
            </div>
        </div>
    }
}
