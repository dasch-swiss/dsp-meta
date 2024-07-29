use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    #[serde(rename = "_page")]
    pub page: usize,
    #[serde(rename = "_limit")]
    pub limit: usize,
}
impl Default for Pagination {
    fn default() -> Self {
        Pagination { page: 1, limit: 10 }
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct Filter {
    #[serde(rename = "q")]
    pub query: Option<String>,
    #[serde(rename = "filter")]
    pub filter: Option<String>,
}

pub struct Page<T> {
    pub data: Vec<T>,
    pub total: usize,
}
/// The contract for the project metadata repository.
/// It defines the methods that the repository must implement.
/// The trait is generically typed for the entity type `Entity`, the id type `Id`, and
/// the error type `Error`.
pub trait RepositoryContract<Entity, Id, Error> {
    /// Retrieves an entity by its id.
    /// If the entity does not exist, `None` is returned.
    fn find_by_id(&self, id: &Id) -> Result<Option<Entity>, Error>;

    /// Returns all entities with filter and pagination.
    fn find(&self, filter: &Filter, pagination: &Pagination) -> Result<Page<Entity>, Error>;

    /// Returns all entities.
    fn find_all(&self) -> Result<Vec<Entity>, Error>;

    /// Returns the number of entities.
    fn count(&self) -> Result<usize, Error>;
}
