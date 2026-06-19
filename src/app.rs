use leptos::prelude::*;
use thaw::ConfigProvider;

use crate::ui::pages::dashboard::DashboardPage;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ConfigProvider>
            <DashboardPage />
        </ConfigProvider>
    }
}
