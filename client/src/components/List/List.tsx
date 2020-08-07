import React from 'react';

type ListProps<T> = {
  items: T[];
  renderItem: (item: T) => React.ReactNode;
};

const List = <T extends {}>({ items, renderItem }: ListProps<T>) => (
  <>{items.map(renderItem)}</>
);

export default List;
