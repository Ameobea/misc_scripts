import * as R from 'ramda';
import React, { useContext, useState, useEffect, useRef } from 'react';
import { Table, Input, Button, Form } from 'antd';
import 'antd/dist/antd.css';

import { Item, formatAmount, parseAmount } from './App';

const EditableContext = React.createContext<any>(null);

interface EditableRowProps {
  index: number;
}

const EditableRow: React.FC<EditableRowProps> = ({ index, ...props }) => {
  const [form] = Form.useForm();

  return (
    <Form form={form} component={false}>
      <EditableContext.Provider value={form}>
        <tr {...props} />
      </EditableContext.Provider>
    </Form>
  );
};

interface EditableCellProps {
  title: React.ReactNode;
  editable: boolean;
  children: React.ReactNode;
  dataIndex: keyof Item;
  record: Item;
  handleSave: (record: Item) => void;
}

const EditableCell: React.FC<EditableCellProps> = ({
  title,
  editable,
  children,
  dataIndex,
  record,
  handleSave,
  ...restProps
}) => {
  const [editing, setEditing] = useState(false);
  const inputRef = useRef<Input | null>(null);
  const form = useContext(EditableContext);

  useEffect(() => {
    if (editing && inputRef.current) {
      inputRef.current.focus();
    }
  }, [editing]);

  const toggleEdit = () => {
    setEditing(!editing);
    form.setFieldsValue({ [dataIndex]: record[dataIndex] });
  };

  const save = async (e: any) => {
    try {
      const values = await form.validateFields();

      toggleEdit();
      handleSave({ ...record, ...values });
    } catch (errInfo) {
      console.log('Save failed:', errInfo);
    }
  };

  let childNode = children;

  if (editable) {
    childNode = editing ? (
      <Form.Item
        style={{ margin: 0 }}
        name={dataIndex}
        rules={[
          {
            required: true,
            message: `${title} is required.`,
          },
        ]}
      >
        <Input ref={inputRef} onPressEnter={save} onBlur={save} />
      </Form.Item>
    ) : (
      <div className="editable-cell-value-wrap" style={{ paddingRight: 24 }} onClick={toggleEdit}>
        {children}
      </div>
    );
  }

  return <td {...restProps}>{childNode}</td>;
};

interface EditableTableProps {
  style?: React.CSSProperties;
  onChange: (rows: Item[]) => void;
}

class EditableTable extends React.Component<
  EditableTableProps,
  { dataSource: Item[]; count: number }
> {
  private columns: { title: string; dataIndex: keyof Item; width?: string; editable?: boolean }[];

  constructor(props: EditableTableProps) {
    super(props);
    this.columns = [
      {
        title: 'Name',
        dataIndex: 'name',
        width: '30%',
        editable: true,
      },
      {
        title: 'Price Per',
        dataIndex: 'pricePer',
        editable: true,
      },
      {
        title: 'Quantity',
        dataIndex: 'quantity',
        editable: true,
      },
      {
        title: 'Total',
        dataIndex: 'totalValue',
        editable: false,
      },
    ];

    this.state = {
      dataSource: [
        {
          key: '0',
          name: 'mahogany logs',
          pricePer: 388,
          quantity: '5.5k',
          totalValue: formatAmount(388 * 5500),
        },
      ],
      count: 1,
    };
  }

  handleDelete = (key: Item['key']) => {
    const dataSource = [...this.state.dataSource];
    const newData = dataSource.filter(item => item.key !== key);
    this.setState({ dataSource: newData });
    this.props.onChange(newData);
  };

  handleAdd = () => {
    const { count, dataSource } = this.state;
    const newData: Item = {
      key: count.toString(),
      name: '-',
      pricePer: 10,
      quantity: 1000,
      totalValue: formatAmount(10 * 1000),
    };
    this.setState({
      dataSource: [...dataSource, newData],
      count: count + 1,
    });
    this.props.onChange([...dataSource, newData]);
  };

  handleSave = (row: Item) => {
    const newData = [...this.state.dataSource];
    const index = newData.findIndex(item => row.key === item.key);
    row.totalValue =
      !R.isNil(row.pricePer) && !R.isNil(row.quantity)
        ? formatAmount(parseAmount(row.pricePer) * parseAmount(row.quantity))
        : '--';
    const item = newData[index];
    newData.splice(index, 1, {
      ...item,
      ...row,
    });
    this.setState({ dataSource: newData });
    this.props.onChange(newData);
  };

  render() {
    const components = {
      body: {
        row: EditableRow,
        cell: EditableCell,
      },
    };

    const columns = this.columns.map(col => {
      if (!col.editable) {
        return col;
      }

      return {
        ...col,
        onCell: (record: any) => ({
          record,
          editable: col.editable,
          dataIndex: col.dataIndex,
          title: col.title,
          handleSave: this.handleSave,
        }),
      };
    });

    return (
      <div style={this.props.style}>
        <Table
          components={components as any}
          rowClassName={() => 'editable-row'}
          bordered
          dataSource={this.state.dataSource}
          columns={columns}
          pagination={false}
        />
        <Button onClick={this.handleAdd} type="primary" style={{ marginBottom: 16, marginTop: 14 }}>
          Add a row
        </Button>
      </div>
    );
  }
}

export default EditableTable;
