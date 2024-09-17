# Adding Metadata

To add metadata to the DSP-META repository,
simply add the metadata JSON file to the folder `data/json/` in the repository.

The JSON file must conform to the schema of the metadata:

- If the project is marked as "Ongoing", use the draft schema, found in `resources/schema-metadata-draft.json`.  
  This schema is less strict and allows for incomplete metadata.
- If the project is marked as "Finished", use the final schema, found in `resources/schema-metadata-final.json`.
  This schema is strict and requires all fields to be filled out.

Once the data is added, open a pull request to the repository. 
The PR name must conform to the conventional commit format,
using the prefix `data`.  
E.g. `data: Add metadata for project XYZ`.

To deploy the newly added metadata to the DSP Metadata Browser,
a new release must be created and deployed to the server.
