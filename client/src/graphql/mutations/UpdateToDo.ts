import gql from 'graphql-tag';

export const updateToDo = gql`
  mutation UpdateToDo($id: String!, $label: String, $done: Boolean) {
    updateToDo(id: $id, label: $label, done: $done) {
      id
      label
      done
    }
  }
`;
