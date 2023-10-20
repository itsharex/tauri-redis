import { type KeyInfo } from '@/store/key'
import request from '@/utils/request'
import { PlusOutlined } from '@ant-design/icons'
import { Form, Input, Select, Tooltip } from 'antd'
import React from 'react'
import { useTranslation } from 'react-i18next'
import useKeyTypes from '@/hooks/useKeyTypes'
import ModalForm from '@/components/ModalForm'
import TopKItem from './components/TopkItem'
import ArrayItem from './components/ArrayItem'
import FiledItem from './components/FieldItem'
import ScoreItem from './components/ScoreItem'
import { type FormInstance } from 'antd/lib'
import FieldInput from '@/components/FieldInput'

const Plus: React.FC<{
  onSuccess: (name: string) => void
  info: KeyInfo
}> = (props) => {
  const { t } = useTranslation()

  const keyTypes = useKeyTypes()

  const form = React.useRef<FormInstance>(null)

  const [types, setTypes] = React.useState<APP.Key['types']>()

  const additionFormItem = React.useMemo(() => {
    switch (types) {
      case 'TopK-TYPE': {
        form.current?.setFieldValue('value', undefined)
        return <TopKItem />
      }
      case 'string':
      case 'ReJSON-RL': {
        form.current?.setFieldValue('value', undefined)
        return (
          <Form.Item
            name={'value'}
            label={t('Value')}
            rules={[{ required: true }]}
          >
            <FieldInput />
          </Form.Item>
        )
      }
      case 'zset': {
        form.current?.setFieldValue('value', [
          {
            field: undefined,
            value: undefined
          }
        ])
        return <ScoreItem />
      }
      case 'hash': {
        form.current?.setFieldValue('value', [
          {
            field: undefined,
            value: undefined
          }
        ])
        return <FiledItem />
      }
      case 'list':
      case 'set': {
        form.current?.setFieldValue('value', [undefined])
        return <ArrayItem />
      }
    }
    return <></>
  }, [t, types])

  const path = React.useMemo(() => {
    switch (types) {
      case 'TopK-TYPE': {
        return 'topk/reserve'
      }
      case 'set': {
        return 'key/set/sadd'
      }
      case 'list': {
        return 'key/list/lpush'
      }
      case 'hash': {
        return 'key/hash/hset'
      }
      case 'zset': {
        return 'key/zset/zadd'
      }
      default: {
        return 'key/add'
      }
    }
  }, [types])

  const onValueChange = React.useCallback((e: any) => {
    if (e.types !== undefined) {
      setTypes(e.types)
      switch (e.types as APP.Key['types']) {
        case 'TopK-TYPE': {
          form.current?.setFieldsValue({
            width: 8,
            depth: 7,
            decay: 0.9
          })
          break
        }
      }
    }
  }, [])

  return (
    <ModalForm
      ref={form}
      onValueChange={onValueChange}
      trigger={
        <Tooltip title={t('New Key')} placement="bottom">
          <PlusOutlined className="hover:cursor-pointer text-lg"></PlusOutlined>
        </Tooltip>
      }
      title={t('New Key')}
      onCancel={() => {
        setTypes(undefined)
      }}
      onSubmit={async (v) => {
        await request(path, props.info.connection.id, {
          db: props.info.db,
          ...v
        })
        props.onSuccess(v.name)
        setTypes(undefined)
      }}
    >
      <Form.Item
        name="types"
        label={t('Key Type')}
        rules={[{ required: true }]}
      >
        <Select
          options={keyTypes}
          placeholder={t('Please Select {{name}}', {
            name: t('Key Type')
          })}
        ></Select>
      </Form.Item>
      <Form.Item name="name" label={t('Key Name')} rules={[{ required: true }]}>
        <Input
          placeholder={t('Please Enter {{name}}', {
            name: t('Key Name')
          }).toString()}
        ></Input>
      </Form.Item>
      {additionFormItem}
    </ModalForm>
  )
}
export default Plus