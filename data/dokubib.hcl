version = 1

project "0804" {
  created_at  = 1630601300976368000
  created_by  = "dsp-metadata-gui"
  shortcode   = "0804"
  name        = "Bilddatenbank Bibliothek St. Moritz"

  alternativeName "1" {
    de = "Dokumentationsbibliothek St. Moritz"
    en = "St. Moritz Documentation Library"
  }

  teaser_text = "Bibliothek St. Moritz Dokumentation is the local history archive of the community of St. Moritz, Switzerland."

  description {
    en = "Bibliothek St. Moritz Dokumentation is the local history archive of the community of St. Moritz, Switzerland. It’s collection contains publications, manuscripts and audiovisual documents of the touristic development of St. Moritz",
    de = "Die Bibliothek St. Moritz Dokumentation ist das Lokalarchiv der Gemeinde St. Moritz. Ihre Sammlung umfasst Publikationen, Manuskripte und audiovisuelle Dokumente zur touristischen Entwicklung von St. Moritz."
  }

  url "https://data.dasch.swiss/dokubib/" {
    text = "Project Website"
  }

  how_to_cite = "Dokumentationsbibliothek St. Moritz"
  start_date  = "2021-01-01"
  end_date    = "2021-12-31"

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

  discipline "1" {
    skos "https://skos.um.es/unesco6/5501" {
      text = "Local history"
    }
  }
  discipline "2" {
    skos "https://skos.um.es/unesco6/5502" {
      text = "Regional history"
    }
  }
  discipline "3" {
    snf "10302" {
      en = "Swiss history"
      de = "Schweizer Geschichte"
      url "1" "https://www.snf.ch/SiteCollectionDocuments/allg_disziplinenliste.pdf" {
        text = "SNF Discipline List"
      }
    }
  }

  spatial_coverage {
    geonames "https://www.geonames.org/2658822" {
      text = "St. Moritz"
    }
  }
  temporal_coverage {
    chronontology "https://chronontology.dainst.org/period/INtagfT8h7Fs" {
      text = "20th and 21st Centuries"
    }
  }
  temporal_coverage {
    chronontology "https://chronontology.dainst.org/period/kqORhO4TGm4n" {
      text = "20th Century (1900 - 1999)"
    }
  }

  publication "1" {
    text = "Bibliothek St. Moritz Dokumentation"
  }
  publication "2" {
    text = "Doe, J. (2020): Some publication. In: Journal for sample publications. Vol. 100.1, p. 1-32."
    url  = "https://www.stmoritz.ch/"
  }

  contact_point "1" {
    id = "gemeinde_st_moritz"
  }

  dmp {
    available = true
    url       = "https://data.dasch.swiss/dokubib/dmp.pdf"
  }
}

dataset "1" {
  created_at        = "1630601300976368000"
  created_by        = "dsp-metadata-gui"
  access_conditions = Restricted
  how_to_cite       = "Dokumentationsbibliothek St. Moritz"
  status            = Ongoing,
  title             = "Dokumentationsbibliothek St. Moritz Bilddatenbank",
  type_of_data      = [
    Image,
    Text
  ]

  abstract {
    en = "Bilddatenbank makes accessible the collection of historic photographs and other graphical representation of St. Moritz Dokumentationsbibliothek"
  }

  language "1" {
    de = "Deutsch"
  }
  language "2" {
    en = "German"
  }
  language "3" {
    fr = "Allemand"
  }

  attribution "1" "biblio_stmoritz" {
    roles = [
      Creator,
      Publisher
    ]
  }
}

grant "1" {
  name   = "Ordinary Budget"
  funder = "gemeinde_st_moritz" // reference to organization
}

grant "2" {
  name   = "Project Funding"
  funder = "snf" // reference to organization
}

organization "gemeinde_st_moritz" {
  created_at = "1630601301506212000",
  created_by = "dsp-metadata-gui",
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

  url "1" "https://www.biblio-stmoritz.ch" {
    text = "www.biblio-stmoritz.ch"
  }

  address {
    street      = "Plazza da Scoula 14",
    postal_code = "7500",
    locality    = "St. Moritz",
    country     = "Switzerland"
  }
}

organization "snf" {
  name = "Swiss National Science Foundation (SNSF)"
}
