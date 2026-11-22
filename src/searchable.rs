/// A Chimithèque type with only an id and a text field, stored in its own
/// table, e.g. a CAS number or a name.
///
/// It lets the database code search, create and return these types with the
/// same functions.
///
/// # Security
///
/// The table, id field and text field names are written as is into SQL
/// queries. They must be fixed identifiers written in the code, never values
/// coming from a request or another user input.
pub trait Searchable {
    /// Returns a new, empty item of the same type.
    #[must_use]
    fn create(&self) -> Self;

    /// Sets whether a search matches the text exactly, instead of containing
    /// it.
    fn set_exact_search(&mut self, match_exact_search: bool);
    /// Returns whether a search matches the text exactly.
    fn get_exact_search(&self) -> bool;

    /// Returns the name of the database table, e.g. `cas_number`.
    fn get_table_name(&self) -> String;

    /// Returns the name of the id column, e.g. `cas_number_id`.
    fn get_id_field_name(&self) -> String;
    /// Returns the name of the text column, e.g. `cas_number_label`.
    fn get_text_field_name(&self) -> String;

    /// Sets the id.
    fn set_id_field(&mut self, id: u64);
    /// Sets the text.
    fn set_text_field(&mut self, text: &str);

    /// Returns the id; `None` for an item not saved yet.
    fn get_id(&self) -> Option<u64>;
    /// Returns the text, e.g. a CAS number.
    fn get_text(&self) -> String;
}
