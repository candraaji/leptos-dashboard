use crate::app::components::DashboardWidget;
use crate::app::models::Person;
use charts_rs::{BarChart, Color, Series, THEME_DARK};
use leptos::*;
use num_format::{Buffer, Locale};

#[component]
pub fn DashboardChart(persons_data: Vec<Person>) -> impl IntoView {
    
    let team_count: String = persons_data.len().to_string();


    let mut total_cost: i32 = 0;

    
    let mut latest_member: String = String::new();
    let mut counter = 0;

    let mut data_vec = Vec::new();
    let mut count_vec: Vec<f32> = Vec::new();


    for person in persons_data {
        if counter == 0 {
            latest_member = person.name;
        }

        // adding this persons' compensation to the total team cost
        total_cost += person.compensation;

        // if the vector for displaying the title doesn't containe this person's
        // title yet
        if !data_vec.contains(&person.title) {
            // we add it to the vector
            data_vec.push(person.title);

            // we also add 1 to the count vector
            count_vec.push(1.0);
        } else {
            // if this title has previously been added to the titles vector
            // we get the index of the title in the titles vector
            let index = data_vec
                .iter()
                .position(|title| title == &person.title)
                .unwrap();

            // we also get the number in the vector using that index
            let num_at_index = count_vec[index];

            // then we increment it by 1
            count_vec[index] = num_at_index + 1.0;
        }

        counter = counter + 1;
    }

    let mut bar_series = Series::new(String::new(), count_vec);
    bar_series.label_show = true;

    let mut bar_chart = BarChart::new_with_theme(vec![bar_series], data_vec, THEME_DARK);
    bar_chart.font_family = String::from("Noto Sans SC");
    bar_chart.background_color = Color::transparent();
    bar_chart.width = 832.0;
    bar_chart.height = 500.0;

    bar_chart.y_axis_hidden = true;

    let mut buf = Buffer::default();
    buf.write_formatted(&total_cost, &Locale::en);
    let total_cost_str = format!("${}", buf.as_str());

    view! {

        <div class="w-full flex flex-col max-w-[64rem] mx-auto pt-8 mb-10">
            <div class="w-full h-20 grid grid-cols-3 gap-4 mx-auto px-2 max-w-[53rem]">
                <DashboardWidget title="Team Members" value=&team_count />
                <DashboardWidget title="Monthly Team Cost" value=&total_cost_str />
                <DashboardWidget title="Just Joined" value=&latest_member />
            </div>

            <div class="max-w-[53rem] mx-auto w-full flex flex-col mt-14 pb-12">
                <div class="w-full max-w-[53rem] h-20 bg-black-200 rounded py-10 px-4 pb-10" inner_html=&bar_chart.svg().unwrap()></div>
            </div>
        </div>
    }
    .into_view()
}