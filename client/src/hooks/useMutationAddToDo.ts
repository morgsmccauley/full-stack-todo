import { useMutation } from '@apollo/client';
import gql from 'graphql-tag';

import {
  addToDo as addToDoMutation,
  AddToDo,
  AddToDoVariables,
} from '../graphql/mutations';

export const useMutationAddToDo = () =>
  useMutation<AddToDo, AddToDoVariables>(addToDoMutation, {
    update(cache, { data }) {
      cache.modify({
        fields: {
          toDos(existingToDos) {
            const toDoRef = cache.writeFragment({
              data: data?.addToDo,
              fragment: gql`
                fragment NewToDo on ToDo {
                  id
                  label
                  done
                }
              `,
            });
            return [...existingToDos, toDoRef];
          },
        },
      });
    },
  });
