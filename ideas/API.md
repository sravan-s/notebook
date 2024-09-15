Nb.set and Nb.get will be the bread and butter to move data between paragrpahs
Lets not share execution contexts between paragrpahs. Imagine them as seperate
scripts.

We need to reserve `Nb` as reserved variable
`Nb.set({type, key, value})`
`Nb.get(key)`
`Nb.put({ data, type, schema })`, type = CSV / JSON / Markdown / String; data is string
When type is `CSV / JSON` , we can present charts.
Lets force user add schema for `JSON`
For example -> 
Nb.put(data: Data)
Data = { type: 'json', schema: string, data: string } // Lets do schema later
  | { type: 'json', data: string }
  | { type: 'md', data: string }
  | { type: 'string', data: string }

I need to implement a keyvalue store for running noetbooks