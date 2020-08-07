import gql from 'graphql-tag';

export const addToDo = gql`
  mutation AddToDo($label: String!) {
    addToDo(label: $label) {
      id
      label
      done
    }
  }
`;
