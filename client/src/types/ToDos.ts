/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: ToDos
// ====================================================

export interface ToDos_toDos {
  __typename: "ToDo";
  id: string;
  label: string;
  done: boolean;
}

export interface ToDos {
  toDos: ToDos_toDos[];
}
