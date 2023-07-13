version = 1

project "081C" {
  created_at  = 1630601274523025000
  created_by  = "dsp-metadata-gui"
  short_code  = "081C"
  teaser_text = "The database documents the different kinds of spectacles such as theatre plays, operas, ballets, or concerts that took place in the Hôtel de Musique in Bern between 1766 and 1905."
  how_to_cite = "HdM-Bern"
  start_date  = "2009-04-01"
  datasets    = ["081C-dataset-0000"]
  funders     = ["081C-organization-000"]

  name "1" {
    de = "Hôtel de Musique Bern"
    en = "Hôtel de Musique Bern"
    fr = "Hôtel de Musique Bern"
  }

  description {
    en = "The database documents the events that took place in the Hôtel de Musique in Bern between 1766 and 1905. The repertoire was constituted by different kinds of spectacles like theatre plays, operas, ballets, concerts, dance parties, acrobatic performances, conferences or magicians. The list reconstructs the lifely and colourful theatre culture of Bern in the 19th Century."
  }

  keyword {
    en = "19 Century"
  }
  keyword {
    de = "Bern"
  }
  keyword {
    en = "Concert"
  }
  keyword {
    en = "Music"
  }
  keyword {
    en = "Musicology"
  }
  keyword {
    en = "Opera"
  }
  keyword {
    en = "Spectales"
  }
  keyword {
    en = "Switzerland"
  }
  keyword {
    en = "Theater history"
  }
  keyword {
    en = "Theatre"
  }

  discipline "1" {
    snf "10302" {
      de = "Schweizer Geschichte"
      url "1" "https://www.snf.ch/SiteCollectionDocuments/allg_disziplinenliste.pdf" {
        text = "SNF Disziplinenliste"
      }
    }
  }
  discipline "2" {
    snf "10405" {
      de = "Musikologie"
      url "1" "https://www.snf.ch/SiteCollectionDocuments/allg_disziplinenliste.pdf" {
        text = "SNF Disziplinenliste"
      }
    }
  }
  discipline "3" {
    snf "10406" {
      de = "Theater-und Filmwissenschaften"
      url "1" "https://www.snf.ch/SiteCollectionDocuments/allg_disziplinenliste.pdf" {
        text = "SNF Disziplinenliste"
      }
    }
  }
  discipline "4" {
    snf "10604" {
      de = "Musik und Theater"
      url "1" "https://www.snf.ch/SiteCollectionDocuments/allg_disziplinenliste.pdf" {
        text = "SNF Disziplinenliste"
      }
    }
  }

  spatial_coverage {
    geonames "https://www.geonames.org/2661552" {
      text = "Bern"
    }
  }

  temporal_coverage {
    periodo "https://n2t.net/ark:/99152/p06c6g3pvr5" {
      text = "Under Mediation act, 1803-1814"
    }
  }
  temporal_coverage {
    periodo "https://n2t.net/ark:/99152/p06c6g3p4cf" {
      text = "Sonderbund, 1845-1847"
    }
  }
  temporal_coverage {
    periodo "https://n2t.net/ark:/99152/p06c6g364np" {
      text = "Helvetic Republic, 1798-1803"
    }
  }
  temporal_coverage {
    text {
      de = "1766-1905"
      en = "1766-1905"
      fr = "1766-1905"
    }
  }

  url "1" "https://admin.dasch.swiss/project/081C" {
    text = "Discover Project Data"
  }
}

dataset "081C-dataset-000" {
  created_at        = 1630601285266958000
  created_by        = "dsp-metadata-gui"
  title             = "Hôtel de Musique Bern"
  status            = Finished
  access_conditions = Open
  date_published    = "2015-04-01"
  how_to_cite       = "HdM-Bern"

  type_of_data = [Text]

  abstract {
    en = "The database documents the events that took place in the Hôtel de Musique in Bern between 1766 and 1905. The repertoire was constituted by different kinds of spectacles like theatre plays, operas, ballets, concerts, dance parties, acrobatic performances, conferences or magicians. The list reconstructs the lifely and colourful theatre culture of Bern in the 19th Century."
  }

  language {
    de = "Deutsch"
    en = "German"
    fr = "allemand"
  }

  license {
    date = "2021-09-02"
    creative_commons "https://creativecommons.org/licenses/by-nc/4.0" {
      text = "CC BY-NC 4.0"
    }
  }

  attribution {
    agent = "081C-organization-000"
    roles = [author]
  }
}

organization "081C-organization-000" {
  created_at = 1630601285796580000
  created_by = "dsp-metadata-gui"
  name       = "Institut für Musikwissenschaft der Universität Bern"
  email      = "urchueguia@musik.unibe.ch"

  url "1" "https://www.musik.unibe.ch" {
    text = "https://www.musik.unibe.ch"
  }

  address {
    street      = "Mittelstr. 43"
    postal_code = "3011"
    locality    = "Bern"
    country     = "Switzerland"
  }
}
