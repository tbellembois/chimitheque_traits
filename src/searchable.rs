pub trait Searchable {
    fn create(&self) -> Self;

    fn set_exact_search(&mut self, match_exact_search: bool);
    fn get_exact_search(&self) -> bool;

    fn get_table_name(&self) -> String;

    fn get_id_field_name(&self) -> String;
    fn get_text_field_name(&self) -> String;

    fn set_id_field(&mut self, id: u64);
    fn set_text_field(&mut self, text: &str);

    fn get_id(&self) -> Option<u64>;
    fn get_text(&self) -> String;
}
