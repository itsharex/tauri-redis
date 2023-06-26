import type React from 'react'
import datetime from './datetime'
import text from './text'
import json from './json'

export interface TypeFormat {
    key: string
    label: string
    format: (content: string) => any
    component?: (content: string) => React.ReactNode
}

const items: Record<string, TypeFormat> = {}
items[datetime.key] = datetime
items[text.key] = text
items[json.key] = json

export default items