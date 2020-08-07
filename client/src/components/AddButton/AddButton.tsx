import React from 'react';
import { Fab } from '@material-ui/core';
import Add from '@material-ui/icons/Add';

type AddButtonProps = {
  onClick: () => void;
};

const AddButton = ({ onClick }: AddButtonProps) => (
  <Fab onClick={onClick}>
    <Add />
  </Fab>
);

export default AddButton;
