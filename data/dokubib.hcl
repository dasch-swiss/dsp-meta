version = 1

project {
  created_at  = 1630601300976368000
  created_by  = "dsp-metadata-gui"
  shortcode   = "0804"
  name        = "Bilddatenbank Bibliothek St. Moritz"

  teaser_text = "Bibliothek St. Moritz Dokumentation is the local history archive of the community of St. Moritz, Switzerland."

  description {
    en = "Bibliothek St. Moritz Dokumentation is the local history archive of the community of St. Moritz, Switzerland. It’s collection contains publications, manuscripts and audiovisual documents of the touristic development of St. Moritz"
  }

  url {
    href = "https://data.dasch.swiss/dokubib/"
    label = "Project Website"
  }

  how_to_cite = "Dokumentationsbibliothek St. Moritz"
  start_date  = "2021-01-01"
  end_date    = "2021-12-31"

  status = "Ongoing"

  keyword {
    en = "local history"
    de = "Lokalgeschichte"
  }
  keyword {
    en = "regional history"
    de = "Regionalgeschichte"
  }
  keyword {
    en = "tourism"
    de = "Tourismus"
  }
  keyword {
    en = "St. Moritz"
    de = "St. Moritz"
  }
  keyword {
    en = "Switzerland"
    de = "Schweiz"
  }

  discipline skos {
    ref_id = "https://skos.um.es/unesco6/5501"
    description = "Local history"
    url = "https://skos.um.es/unesco6/5501"
  }

  spatial_coverage geonames {
    ref_id = "https://www.geonames.org/2658822"
    description = "St. Moritz"
    url = "https://www.geonames.org/2658822"
  }
  temporal_coverage chronontology {
    ref_id = "https://chronontology.dainst.org/period/INtagfT8h7Fs"
    description = "20th and 21st Centuries"
    url = "https://chronontology.dainst.org/period/INtagfT8h7Fs"
  }
  temporal_coverage chronontology {
    ref_id = "https://chronontology.dainst.org/period/kqORhO4TGm4n"
    description = "20th Century (1900 - 1999)"
    url = "https://chronontology.dainst.org/period/kqORhO4TGm4n"
  }

  publication {
    text = "Bibliothek St. Moritz Dokumentation"
  }

  // reference to person or organization (0-1)
  contact_point = "gemeinde_st_moritz"
}

dataset {
  created_at        = "1630601300976368000"
  created_by        = "dsp-metadata-gui"
  access_conditions = Restricted
  how_to_cite       = "Dokumentationsbibliothek St. Moritz"
  status            = Ongoing
  title             = "Dokumentationsbibliothek St. Moritz Bilddatenbank"
  type_of_data      = [Image, Text]

  abstract {
    en = "Bilddatenbank makes accessible the collection of historic photographs and other graphical representation of St. Moritz Dokumentationsbibliothek"
  }

  language {
    de = "Deutsch"
  }
  language {
    en = "German"
  }
  language {
    fr = "Allemand"
  }

  attribution "biblio_stmoritz" {
    roles = [
      Creator,
      Publisher
    ]
  }
}

dmp {
  available = false
}

grant {
  name   = "Ordinary Budget"
  funder = "gemeinde_st_moritz" // reference to person or organization
}

organization "gemeinde_st_moritz" {
  created_at = "1630601301506212000"
  created_by = "dsp-metadata-gui"
  name       = "Gemeinde St. Moritz"

  address {
    street      = ""
    postal_code = "7500"
    locality    = "St. Moritz"
    country     = "Switzerland"
  }
}

organization "biblio_stmoritz" {
  created_at = "1630601301561696000"
  created_by = "dsp-metadata-gui"
  email      = "doku@biblio-stmoritz.ch"
  name       = "Dokumentationsbibliothek St. Moritz"

  url {
    href = "https://www.biblio-stmoritz.ch"
    label = "www.biblio-stmoritz.ch"
  }

  address {
    street      = "Plazza da Scoula 14"
    postal_code = "7500"
    locality    = "St. Moritz"
    country     = "Switzerland"
  }
}
