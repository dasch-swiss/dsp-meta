# DSP Metadata

The dsp-meta repository contains the code of the [DSP Metadata Browser](https://meta.dasch.swiss),
as well all metadata from projects deposited on the DaSCH Service Platform (DSP).

## DSP Metadata

This documentation provides an overview of the metadata model used by the DSP to manage and describe
research data in the humanities. Our vision is to fully capture the provenance of research data—detailing
its origins, how it was created, and how it has been used over time.

Humanities research projects are inherently diverse and often span multiple years or even decades.
Many of these projects receive funding from various grants and different funders throughout their lifecycle.
Additionally, the researchers involved in creating and reusing the data may change over time, reflecting
the evolving nature of academic collaboration.

Understanding the complex history of research data is crucial for transparency, reproducibility,
and future scholarship. The DSP metadata model is designed to accommodate this complexity by meticulously
recording the provenance of data. It tracks:
	•	Funding Sources: Documenting the multiple grants and funders that have supported the project over time.
	•	Research Personnel: Keeping a record of all researchers who have contributed to or utilized the data,
	acknowledging the shifts in team composition.
	•	Data Lifecycle: Outlining how the data was created, modified, and reused, providing a comprehensive
	view of its evolution.

By capturing this rich contextual information, we aim to provide a robust framework that supports the integrity
and longevity of humanities research data. Whether you are a researcher contributing new data or a scholar
exploring existing datasets, this documentation will guide you through our metadata practices and help you
understand the stories behind the data.

### Consuming Metadata

If you are interested in viewing the metadata in human-readable form,
you can visit the [DSP Metadata Browser](https://meta.dasch.swiss).

If you are interested in re-using our metadata, you can find extensive documentation [here](data/current-datamodel.md),
and the work-in-progress documentation of our future data-model [here](data/future-datamodel.md).

The metadata itself can be found [here](https://github.com/dasch-swiss/dsp-meta/tree/main/data/json)
or requested over the API as described [here](data/api.md).

### Adding Metadata

For adding metadata, please see [here](data/adding-metadata.md).

## Code Documentation

For documentation on the code of the DSP Metadata Browser, please see [here](code/overview.md).
