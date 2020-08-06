import gql from 'graphql-tag';

export const AllToDos = gql`
  query AllToDos {
    toDos {
      id
      label
      done
    }
  }
`;
