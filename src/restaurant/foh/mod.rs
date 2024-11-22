pub mod hosting;
pub mod serving;

pub fn process_visit() {
    hosting::welcome();
    hosting::add_to_wait_list();
    hosting::seat_at_table();
    serving::take_order();
    serving::serve_order();
    super::boh::cook_order();
    serving::take_complaint();
    super::boh::fix_incorrect_order();
    serving::take_payment();
}
