version = 1

project {
  created_at  = 1630601274523025000
  created_by  = "dsp-metadata-gui"
  shortcode   = "081C"
  name        = "Hôtel de Musique Bern"

  teaser_text = "The database documents the different kinds of spectacles such as theatre plays, operas, ballets, or concerts that took place in the Hôtel de Musique in Bern between 1766 and 1905."

  description {
    en = "The database documents the events that took place in the Hôtel de Musique in Bern between 1766 and 1905. The repertoire was constituted by different kinds of spectacles like theatre plays, operas, ballets, concerts, dance parties, acrobatic performances, conferences or magicians. The list reconstructs the lifely and colourful theatre culture of Bern in the 19th Century."
  }

  url {
    href = "https://admin.dasch.swiss/project/081C"
    label = "Discover Project Data"
  }

  how_to_cite = "HdM-Bern"
  start_date  = "2009-04-01"
  end_date    = "2015-04-01"

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

  discipline snf {
    ref_id = "10302"
    description = "Schweizer Geschichte"
    url = "https://www.snf.ch/SiteCollectionDocuments/allg_disziplinenliste.pdf"
  }
  discipline snf {
    ref_id = "10405"
    description = "Musikologie"
    url = "https://www.snf.ch/SiteCollectionDocuments/allg_disziplinenliste.pdf"
  }
  discipline snf {
    ref_id = "10406"
    description = "Theater-und Filmwissenschaften"
    url  = "https://www.snf.ch/SiteCollectionDocuments/allg_disziplinenliste.pdf"
  }
  discipline snf {
    ref_id = "10604"
    description = "Musik und Theater"
    url = "https://www.snf.ch/SiteCollectionDocuments/allg_disziplinenliste.pdf"
  }

  spatial_coverage geonames {
    ref_id = "https://www.geonames.org/2661552"
    description = "Bern"
    url = "https://www.geonames.org/2661552"
  }

  temporal_coverage periodo {
    ref_id = "https://n2t.net/ark:/99152/p06c6g3pvr5"
    description = "Under Mediation act, 1803-1814"
    url = "https://n2t.net/ark:/99152/p06c6g3pvr5"
  }
  temporal_coverage periodo {
    ref_id = "https://n2t.net/ark:/99152/p06c6g3p4cf"
    description = "Sonderbund, 1845-1847"
    url = "https://n2t.net/ark:/99152/p06c6g3p4cf"
  }
  temporal_coverage periodo {
    ref_id = "https://n2t.net/ark:/99152/p06c6g364np"
    description = "Helvetic Republic, 1798-1803"
    url = "https://n2t.net/ark:/99152/p06c6g364np"
  }
  temporal_coverage text {
      de = "1766-1905"
      en = "1766-1905"
      fr = "1766-1905"
  }
}

dataset {
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
    fr = "Allemand"
  }

  license "creative_commons" "https://creativecommons.org/licenses/by-nc/4.0" {
    date = "2021-09-02"
    text = "CC BY-NC 4.0"
  }

  // reference to person or organization
  attribution "musik_unibe" {
    roles = [Author]
  }
}

grant {
  name   = "Ordinary Budget"
  funder = "musik_unibe" // reference to organization
}

organization "musik_unibe" {
  created_at = 1630601285796580000
  created_by = "dsp-metadata-gui"
  name       = "Institut für Musikwissenschaft der Universität Bern"
  email      = "urchueguia@musik.unibe.ch"

  url {
    href = "https://www.musik.unibe.ch"
    description = "https://www.musik.unibe.ch"
  }

  address {
    street      = "Mittelstr. 43"
    postal_code = "3011"
    locality    = "Bern"
    country     = "Switzerland"
  }
}
