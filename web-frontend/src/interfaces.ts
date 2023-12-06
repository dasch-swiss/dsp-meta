export interface Category {
  id: number;
  isOpen: boolean;
  name: string;
  sub: string[];
}
export interface PaginationData {
  currentPage: number;
  currentResultsRange: number[];
  totalCount: number;
  totalPages: number;
}

export interface ProjectMetadata {
  description: string;
  id: string;
  status: string;
  name: string;
}

export interface Metadata {
  $schema: String;
  project: Project;
  datasets: Dataset[];
  persons?: Person[];
  organizations: Organization[];
  grants: Grant[];
}

export interface Project {
  __type: "Project";
  __id: string;
  __createdAt: string;
  __createdBy: string;
  shortcode: string;
  name: string;
  description: Text;
  howToCite: string;
  startDate: string;
  url: URL;
  teaserText: string;
  datasets: string[];
  keywords: Text[];
  disciplines: (URL | Text)[];
  temporalCoverage: (URL | Text)[];
  spatialCoverage: URL[];
  funders: string[];
  dataManagementPlan?: DataManagementPlan;
  endDate?: string;
  contactPoint?: string;
  secondaryURL?: URL;
  publications?: string[];
  grants?: string[];
  alternativeNames?: Text[];
}

export interface Dataset {
  __type: "Dataset";
  __id: string;
  __createdAt: string;
  __createdBy: string;
  title: string;
  accessConditions: string;
  howToCite: string;
  status: string;
  abstracts: (Text | URL)[];
  typeOfData: string[];
  licenses: License[];
  languages: Text[];
  attributions: Attribution[];
  datePublished?: string;
  dateCreated?: string;
  dateModified?: string;
  distribution?: URL;
  alternativeTitles?: Text[];
  urls?: URL[];
  additional?: (Text | URL)[];
}

export interface License {
  __type: "License";
  date: string;
  license: URL;
  details?: string;
}

export interface Attribution {
  __type: "Attribution";
  agent: string;
  roles: string[];
}

export interface Address {
  __type: "Address";
  street: string;
  additional?: string;
  postalCode: string;
  locality: string;
  canton?: string;
  country: string;
}

export interface Person {
  __type: "Person";
  __id: string;
  __createdAt: string;
  __createdBy: string;
  email: string;
  jobTitles: string[];
  givenNames: string[];
  familyNames: string[];
  affiliation: string[];
  secondaryEmail?: string;
  address?: Address;
  authorityRefs?: URL[];
}

export interface URL {
  __type: "URL";
  text?: string;
  type: string;
  url: string;
}

export interface Text {
  __type: "Text";
  [lang: string]: string
}

export interface Organization {
  __type: "Organization";
  __id: string;
  __created: string;
  __modified: string;
  name: string;
  url?: URL;
  address?: Address;
  email?: string;
  alternativeNames?: Text[];
  authorityRefs?: URL[];
}

export interface Grant {
  __type: "Grant";
  __id: string;
  __created: string;
  __modified: string;
  funders: string[];
  number?: string;
  name?: string;
  url?: URL;
}

export interface DataManagementPlan {
  __type: "DataManagementPlan";
  available?: boolean;
  url?: URL;
}
