version = 1

project "dsp-0804-project" {
  shortcode = "0804"
  name = "Bilddatenbank Bibliothek St. Moritz"
  description = {
    en = "Bibliothek St. Moritz Dokumentation is the local history archive of the community of St. Moritz, Switzerland. It’s collection contains publications, manuscripts and audiovisual documents of the touristic development of St. Moritz",
    de = "Die Bibliothek St. Moritz Dokumentation ist das Lokalarchiv der Gemeinde St. Moritz. Ihre Sammlung umfasst Publikationen, Manuskripte und audiovisuelle Dokumente zur touristischen Entwicklung von St. Moritz."
  }
  how_to_cite = "Dokumentationsbibliothek St. Moritz"
  teaser_text = "Bibliothek St. Moritz Dokumentation is the local history archive of the community of St. Moritz, Switzerland."
  disciplines {
    skos "https://skos.um.es/unesco6/5501" {
      text: "Local history"
    }

    skos "https://skos.um.es/unesco6/5502" {
      text: "Regional history"
    }
  }
  funders = [
    "dsp-0804-organization-001"
  ]
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
      text: "St. Moritz"
    }
  }
  temporal_coverage {
    chonontology "https://chronontology.dainst.org/period/INtagfT8h7Fs" {
      text: "20th and 21st Centuries"
    }
    chronontology "https://chronontology.dainst.org/period/kqORhO4TGm4n" {
      text: "20th Century (1900 - 1999)"
    }
  }
  url "https://data.dasch.swiss/dokubib/" {
    text: "Project Website"
  }
}
