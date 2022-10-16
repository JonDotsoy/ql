# Search QL

> ☢️ This project is a demonstration concept, it is not recommended for a production project.

An language to make a search instruction. This language provides a set of definitions to write a simply query search and transform on JSON, typescript, etc...


## Demos

### Demo 1

```
"search value"
```

**JSON:**
```json
[
  ["search value"]
]
```

**TypeScript:**
```ts
type Item = {
    title: string
    condition: string
    context: {
        condition: string
    }
}

const defaultField = "title"
function filter(item: Item) {
    return item.title.match("search value")
}
```

### Demo 2

```
"search value condition:"
```

***json:*** 
```json
[
  ["search value condition:"]
]
```

**TypeScript:**
```ts
type Item = {
    title: string
    condition: string
    context: {
        condition: string
    }
}

const defaultField = "title"
function filter(item: Item) {
    return item.title.match("search value condition:")
}
```

### Demo 3

```
"search value" condition: "value"
```

**JSON:**
```json
[
  ["search value"],
  [["condition"],"equal","value"]
]
```

**TypeScript:**

```ts
type Item = {
    title: string
    condition: string
    context: {
        condition: string
    }
}

const defaultField = "title"
function filter(item: Item) {
    return item.title.match("search value")
    && item.condition == "value"
}
```


### Demo 4

```
"search value"
condition: "value"
context.condition > "value"
```

**JSON:**

```json
[
  ["search value"],
  [["condition"],"equal","value"],
  [["context","condition"],"greaterThan","value"]
]
```

**TypeScript:**

```ts
type Item = {
    title: string
    condition: string
    context: {
        condition: string
    }
}

const defaultField = "title"
function filter(item: Item) {
    return item.title.match("search value")
    && item.condition == "value"
    && item.context.condition > "value"
}
```
