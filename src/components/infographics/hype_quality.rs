use leptos::prelude::*;

use crate::hash_router::navigate_hash;
use crate::models::School;

use super::data::compute_hype_quality;

#[component]
pub fn HypeQuality(schools: Vec<School>) -> impl IntoView {
    let data = compute_hype_quality(&schools);

    let correlation_r = data.correlation_r;
    let schools_analyzed = data.schools_analyzed;

    let hidden_gems_view = data
        .hidden_gems
        .iter()
        .map(|entry| {
            let school_id = entry.school_id.clone();
            let name = entry.name.clone();
            let abitur = entry.abitur;
            let demand = entry.demand;

            view! {
                <div class="ig-hype-row">
                    <a
                        class="ig-link ig-hype-name"
                        href="javascript:void(0)"
                        on:click=move |ev: leptos::ev::MouseEvent| {
                            ev.prevent_default();
                            navigate_hash(&format!("/school/{}", school_id), false);
                        }
                    >
                        {name}
                    </a>
                    <span class="ig-hype-abitur ig-green">{format!("{:.2}", abitur)}</span>
                    <span class="ig-hype-demand">{format!("{:.2}x", demand)}</span>
                </div>
            }
        })
        .collect_view();

    let overhyped_view = data
        .overhyped
        .iter()
        .map(|entry| {
            let school_id = entry.school_id.clone();
            let name = entry.name.clone();
            let abitur = entry.abitur;
            let demand = entry.demand;

            view! {
                <div class="ig-hype-row">
                    <a
                        class="ig-link ig-hype-name"
                        href="javascript:void(0)"
                        on:click=move |ev: leptos::ev::MouseEvent| {
                            ev.prevent_default();
                            navigate_hash(&format!("/school/{}", school_id), false);
                        }
                    >
                        {name}
                    </a>
                    <span class="ig-hype-abitur ig-red">{format!("{:.2}", abitur)}</span>
                    <span class="ig-hype-demand">{format!("{:.2}x", demand)}</span>
                </div>
            }
        })
        .collect_view();

    let best_demand = data.best_combo_demand;
    let worst_demand = data.worst_combo_demand;

    let combo_view = match (data.best_combo, data.worst_combo) {
        (Some(best), Some(worst)) => {
            let best_id = best.school_id.clone();
            let best_name = best.name.clone();
            let best_value = best.value;
            let worst_id = worst.school_id.clone();
            let worst_name = worst.name.clone();
            let worst_value = worst.value;

            Some(view! {
                <div class="ig-stats-row">
                    <div class="ig-stat">
                        <span class="ig-stat-label">"Best Combo"</span>
                        <a
                            class="ig-link ig-stat-value"
                            href="javascript:void(0)"
                            on:click=move |ev: leptos::ev::MouseEvent| {
                                ev.prevent_default();
                                navigate_hash(&format!("/school/{}", best_id), false);
                            }
                        >
                            {best_name}
                        </a>
                        <span class="ig-stat-desc">{format!("{:.2} / {:.2}x", best_value, best_demand.unwrap_or(0.0))}</span>
                    </div>
                    <div class="ig-stat">
                        <span class="ig-stat-label">"Worst Combo"</span>
                        <a
                            class="ig-link ig-stat-value"
                            href="javascript:void(0)"
                            on:click=move |ev: leptos::ev::MouseEvent| {
                                ev.prevent_default();
                                navigate_hash(&format!("/school/{}", worst_id), false);
                            }
                        >
                            {worst_name}
                        </a>
                        <span class="ig-stat-desc">{format!("{:.2} / {:.2}x", worst_value, worst_demand.unwrap_or(0.0))}</span>
                    </div>
                    <div class="ig-stat">
                        <span class="ig-stat-value">{schools_analyzed}</span>
                        <span class="ig-stat-label">"Schools Analyzed"</span>
                    </div>
                </div>
            })
        }
        _ => None,
    };

    view! {
        <div class="ig-card">
            <div class="ig-header">
                <span class="ig-eyebrow">"BERLIN GYMNASIEN"</span>
                <h2 class="ig-title">"Does Hype = Quality?"</h2>
                <p class="ig-subtitle">"Correlation between school demand and Abitur performance"</p>
            </div>
            <div class="ig-correlation">
                <span class="ig-correlation-value">{format!("r = {:.2}", correlation_r)}</span>
                <span class="ig-correlation-sig">"significant · p < 0.001"</span>
                <p class="ig-insight-text">"Higher demand → better grades (but not always)"</p>
            </div>
            <div class="ig-section">
                <h3 class="ig-section-title ig-green">"Hidden Gems"</h3>
                <p class="ig-section-hint">"Great Abitur results, low demand"</p>
                {hidden_gems_view}
            </div>
            <div class="ig-section">
                <h3 class="ig-section-title ig-red">"Overhyped?"</h3>
                <p class="ig-section-hint">"High demand, mediocre Abitur results"</p>
                {overhyped_view}
            </div>
            {combo_view}
        </div>
    }
}
