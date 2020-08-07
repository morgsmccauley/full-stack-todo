import { useQuery } from '@apollo/client';
import { allToDos as allToDosQuery, AllToDos } from '../graphql/queries';

export const useQueryAllToDos = () => useQuery<AllToDos>(allToDosQuery);
