/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL mutation operation: UpdateToDo
// ====================================================

export interface UpdateToDo_updateToDo {
  __typename: "ToDo";
  id: string;
  label: string;
  done: boolean;
}

export interface UpdateToDo {
  updateToDo: UpdateToDo_updateToDo;
}

export interface UpdateToDoVariables {
  id: string;
  label?: string | null;
  done?: boolean | null;
}
