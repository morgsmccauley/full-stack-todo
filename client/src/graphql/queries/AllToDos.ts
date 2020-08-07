import gql from 'graphql-tag';

export const allToDos = gql`
  query AllToDos {
    toDos {
      id
      label
      done
    }
  }
`;
