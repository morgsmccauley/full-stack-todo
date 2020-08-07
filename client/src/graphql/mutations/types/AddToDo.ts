/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL mutation operation: AddToDo
// ====================================================

export interface AddToDo_addToDo {
  __typename: "ToDo";
  id: string;
  label: string;
  done: boolean;
}

export interface AddToDo {
  addToDo: AddToDo_addToDo;
}

export interface AddToDoVariables {
  label: string;
}
