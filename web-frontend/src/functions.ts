//  TODO: find better name for this file
import {get} from "svelte/store";
import {projectMetadata as ProjectMetadata} from "./store";
import {handleSnackbar} from "./store";
import type {Grant, Person, Organization, Text} from "./interfaces";

export const copyToClipboard = (what: string) => {
  let text = document.createRange();
  text.selectNode(document.getElementById(what));
  window.getSelection().removeAllRanges();
  window.getSelection().addRange(text);
  document.execCommand('copy');
  window.getSelection().removeAllRanges();
  what = what.split('-').map(word => word.charAt(0).toUpperCase() + word.substring(1)).join(' ')
  handleSnackbar.set({isSnackbar: true, message: `${what} copied successfully!`});
};

export const copyHowToCite = () => {
  copyToClipboard('how-to-cite')
}

export const copyPermalink = () => {
  copyToClipboard('permalink')
}

export function findPersonByID(id: string): Person {
  let persons = get(ProjectMetadata).persons;
  if (persons && persons.length > 0) {
    return persons.find(o => o.__id === id);
  }
}

export function findOrganizationByID(id: string): Organization {
  let x = get(ProjectMetadata).organizations
  return x ? x.find(o => o.__id === id) : undefined;
}

export function findGrantByID(id: string): Grant {
  let x = get(ProjectMetadata).grants
  return x ? x.find(o => o.__id === id) : undefined;
}

export function findObjectByID(id: string): Grant | Person | Organization {
  let o: Grant | Person | Organization;
  o = findPersonByID(id);
  if (o) return o;
  o = findOrganizationByID(id);
  if (o) return o;
  o = findGrantByID(id);
  if (o) return o;
}

export function getText(text: Text, lang?: string) {
  if (!text) {
    return ""
  }

  let langs = Object.keys(text);

  if (langs.length === 0) {
    return ""
  } else if (lang && langs.includes(lang)) {
    return text[lang]
  } else if (langs.includes('en')) {
    return text['en']
  } else {
    return text[langs[0]]
  }
}
