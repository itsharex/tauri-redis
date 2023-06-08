import { Button, Form, Select, InputNumber } from 'antd'
import React from 'react'
import { useTranslation } from 'react-i18next'
import { observer } from 'mobx-react-lite'
import { SettingOutlined } from '@ant-design/icons'
import useStore from '@/hooks/useStore'
import { useForm } from 'antd/es/form/Form'
import lodash from 'lodash'
import CusModal from '../CusModal'

const Index: React.FC = () => {
  const { i18n, t } = useTranslation()

  const store = useStore()

  const [form] = useForm()

  return (
    <>
      <CusModal
        trigger={<Button icon={<SettingOutlined />} size="large"></Button>}
        destroyOnClose
        title={t('Setting')}
        onOk={async () => {
          const data = await form.validateFields()
          store.setting.update(data)
        }}
      >
        <div className="pt-4">
          <Form
            form={form}
            layout="vertical"
            onFieldsChange={(v) => {
              if (v.length > 0) {
                if (lodash.isArray(v[0].name)) {
                  if (v[0].name.includes('locale')) {
                    store.setting.update({
                      locale: v[0].value
                    })
                  }
                }
              }
            }}
            initialValues={{
              ...store.setting.setting
            }}
          >
            <Form.Item name="locale" label={t('Language')}>
              <Select
                options={Object.keys(i18n.store.data).map((v) => {
                  return {
                    value: v,
                    label: i18n.store.data[v].label as string
                  }
                })}
              ></Select>
            </Form.Item>
            <Form.Item
              name="key_count"
              label={t('Key Load Number')}
              tooltip={t('The COUNT option for command SCAN')}
            >
              <InputNumber min={1} />
            </Form.Item>
            <Form.Item
              name="field_count"
              label={t('Field Load Number')}
              tooltip={t('The COUNT option for command HSCAN,SSCAN,ZSCAN')}
            >
              <InputNumber min={1} />
            </Form.Item>
          </Form>
        </div>
      </CusModal>
    </>
  )
}
export default observer(Index)
