version = 1

// Description of the project
project "0804" {
  created_at = "1630601300976368000"
  created_by = "dsp-metadata-gui"
  shortcode = "0804"
  name "1" {
    de = "Bilddatenbank Bibliothek St. Moritz"
  }
  name "2" {
    de = "Bibliothek St. Moritz Dokumentation"
    en = "Bibliothek St. Moritz Dokumentation"
  }
  description = {
    en = "Bibliothek St. Moritz Dokumentation is the local history archive of the community of St. Moritz, Switzerland. It’s collection contains publications, manuscripts and audiovisual documents of the touristic development of St. Moritz",
    de = "Die Bibliothek St. Moritz Dokumentation ist das Lokalarchiv der Gemeinde St. Moritz. Ihre Sammlung umfasst Publikationen, Manuskripte und audiovisuelle Dokumente zur touristischen Entwicklung von St. Moritz."
  }
  start_date = "2021-01-01"
  end_date = "2021-12-31"
  how_to_cite = "Dokumentationsbibliothek St. Moritz"
  teaser_text = "Bibliothek St. Moritz Dokumentation is the local history archive of the community of St. Moritz, Switzerland."
  discipline "1" {
    skos "https://skos.um.es/unesco6/5501" {
      text  = "Local history"
    }
  }
  discipline "2" {
    skos "https://skos.um.es/unesco6/5502" {
      text  = "Regional history"
    }
  }
  discipline "3" {
    snf "10302" {
      en = "Swiss history"
      de = "Schweizer Geschichte"
      url "1" "https://www.snf.ch/SiteCollectionDocuments/allg_disziplinenliste.pdf" {
        text: "SNF Discipline List"
      }
    }
  }
  keywords = {
    en = [
      "local history",
      "regional history",
      "tourism",
      "St. Moritz",
      "Switzerland"
    ],
    de = [
      "Lokalgeschichte",
      "Regionalgeschichte",
      "Tourismus",
      "St. Moritz",
      "Schweiz"
    ]
  }
  spatial_coverage {
    geonames "https://www.geonames.org/2658822" {
      text= "St. Moritz"
    }
  }
  temporal_coverage {
    chronontology "https://chronontology.dainst.org/period/INtagfT8h7Fs" {
      text = "20th and 21st Centuries"
    }
  }
  temporal_coverage {
    chronontology "https://chronontology.dainst.org/period/kqORhO4TGm4n" {
      text= "20th Century (1900 - 1999)"
    }
  }
  url "1" "https://data.dasch.swiss/dokubib/" {
    text= "Project Website"
  }
  url "2" "https://www.stmoritz.ch/" {
    text= "St. Moritz Tourism Website"
  }
  dmp {
    available = true
    url       = "https://data.dasch.swiss/dokubib/dmp.pdf"
  }
  publication "1" {
    text= "Bibliothek St. Moritz Dokumentation"
  }
  publication "2" {
    text= "Doe, J. (2020): Some publication. In: Journal for sample publications. Vol. 100.1, p. 1-32."
    url= "https://www.stmoritz.ch/"
  }
  funder "1" {
    id= "gemeinde_st_moritz"
  }
  funder "2" {
    id= "snf"
  }
  funder "3" {
    text= "Gemeinde St. Moritz"
  }
  contact_point "1" {
    id= "gemeinde_st_moritz"
  }
  grant "1" {
    id= "grant-001"
  }
  grant "2" {
    text= "Die Bibliothek St. Moritz Dokumentation wird von der Gemeinde St. Moritz finanziert."
  }
  dataset "1" {
    id= "dataset-001"
  }
}
