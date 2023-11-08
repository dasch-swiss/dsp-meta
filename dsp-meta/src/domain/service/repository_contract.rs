/// The contract for the project metadata repository.
/// It defines the methods that the repository must implement.
/// The trait is generically typed for the entity type `Entity`, the id type `Id`, and
/// the error type `Error`.
pub trait RepositoryContract<Entity, Id, Error> {
    /// Retrieves an entity by its id.
    /// If the entity does not exist, `None` is returned.
    fn find_by_id(&self, id: &Id) -> Result<Option<Entity>, Error>;

    /// Retrieves all entities identified by the given ids.
    /// If some or all entities for the provided ids do not exist, they are ignored.
    fn find_all_by_id(&self, ids: Vec<&Id>) -> Result<Vec<Entity>, Error> {
        let mut result: Vec<Entity> = vec![];
        for id in ids {
            if let Some(entity) = self.find_by_id(id)? {
                result.push(entity)
            }
        }
        Ok(result)
    }

    /// Checks weather an entity with the given id exists.
    fn exists_by_id(&self, id: &Id) -> Result<bool, Error> {
        match self.find_by_id(id)? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    /// Returns all entities.
    fn find_all(&self) -> Result<Vec<Entity>, Error>;

    /// Return the number of all entities.
    fn count(&self) -> Result<usize, Error> {
        let all_entities = self.find_all()?;
        Ok(all_entities.len())
    }

    /// Saves a given entity and return the saved instance, as the save operation
    /// could have potentially changed the initially provide instance.
    ///
    /// Consumes the provided instance and returns a new one.
    fn save(&self, entity: Entity) -> Result<Entity, Error>;

    fn delete(&self, entity: Entity) -> Result<(), Error>;
}
