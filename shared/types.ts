export type PluginGraphql = {
  input: {
    type: 'aggregateAmcInfo';
    itemIds: string[];
    periodId: string;
    orderType: string | null;
  };

  output: {
    type: 'aggregateAmcInfo';
    result: {
      itemId: string;
      name: string;
      stats: {
        amc: number | null;
        stock: number | null;
        periodEndDate:
          | { type: 'PeriodEnd'; date: string }
          | { type: 'Average' }
          | null;
      };
    }[];
  };
};
