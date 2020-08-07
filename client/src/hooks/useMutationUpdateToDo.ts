import { useMutation } from '@apollo/client';
import {
  updateToDo as updateToDoMutation,
  UpdateToDo,
  UpdateToDoVariables,
} from '../graphql/mutations';

export const useMutationUpdateToDo = () =>
  useMutation<UpdateToDo, UpdateToDoVariables>(updateToDoMutation);
