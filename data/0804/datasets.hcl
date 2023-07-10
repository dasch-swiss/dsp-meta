version = 1

dataset "0804:dataset-001" {
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

  attribution "1" "biblio" {
    roles = [
      Creator,
      Publisher
    ]
  }
}
