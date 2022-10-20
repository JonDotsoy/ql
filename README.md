# Search QL

> ☢️ This project is a demonstration concept, it is not recommended for a production project.

An language to make a search instruction. This language provides a set of definitions to write a simply query search and transform on JSON, typescript, etc...


## Demos

### Demo 1

![](./docs/assets/search%20box%20-%20demo-1.png)

```
search value
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

![](./docs/assets/search%20box%20-%20demo-2.png)

```
search value condition:
```

**JSON:***

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

![](./docs/assets/search%20box%20-%20demo-3.png)

```
search value condition: "value"
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

![](./docs/assets/search%20box%20-%20demo-4.png)

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

## Contribution

Depedencies:
 - cargo-insta: install with `cargo install cargo-insta`

