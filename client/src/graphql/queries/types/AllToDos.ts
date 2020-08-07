/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: AllToDos
// ====================================================

export interface AllToDos_toDos {
  __typename: "ToDo";
  id: string;
  label: string;
  done: boolean;
}

export interface AllToDos {
  toDos: AllToDos_toDos[];
}
