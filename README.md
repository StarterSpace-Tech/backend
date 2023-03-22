# API Documentation

## GET /teams

Returns a list of teams.

```json
[
	{
		"id": "int",
		"rank": "int?",
		"score": "int",
		"stage": "int",
		"name": "string",
		"logo_url": "string?",
		"banner_url": "string?",
		"description": "string",
		"creation_date": "string",
		"location": "string",
		"labels":
		[
			{
				"id": "int",
				"name": "string"
			}
		],
		"persons":
		[
			{
				"id": "int",
				"team_id": "int",
				"name": "string",
				"career": "string",
				"graduation_date": "string (YYYY-MM-DD)",
				"picture_url": "string?",
				"portafolio_url": "string?"
			}
		],
		"badges":
		[
			{
				"id": "int",
				"acquisition_date": "string (YYYY-MM-DD)",
				"badge": {
					"id": "int",
					"name": "string",
					"description": "string",
					"points": "int",
					"category": {
						"id": "int",
						"name": "string"
					}
				}
			}
		]
	}
]
```

## GET /badges

Returns all badges.

```json
[ 
	{
		"id": "int",
		"name": "string",
		"description": "string",
		"points": "int",
		"category": {
			"id": "int",
			"name": "string"
		}
	}
]
```

## GET /labels

Returns all labels.

```json
[
	{
		"id": "int",
		"name": "string"
	}
]
```

## GET /categories

Returns all categories.

```json
[
	{
		"id": "int",
		"name": "string"
	}
]
```

## GET /team/{id}

Returns a team with specific `id`.

```json
{
	"id": "int",
	"rank": "int?",
	"score": "int",
	"stage": "int",
	"name": "string",
	"logo_url": "string?",
	"banner_url": "string?",
	"description": "string",
	"creation_date": "string",
	"location": "string",
	"labels":
	[
		{
			"id": "int",
			"name": "string"
		}
	],
	"persons":
	[
		{
			"id": "int",
			"team_id": "int",
			"name": "string",
			"career": "string",
			"graduation_date": "string (YYYY-MM-DD)",
			"picture_url": "string?",
			"portafolio_url": "string?"
		}
	],
	"badges":
	[
		{
			"id": "int",
			"acquisition_date": "string (YYYY-MM-DD)",
			"badge": {
				"id": "int",
				"name": "string",
				"description": "string",
				"points": "int",
				"category": {
					"id": "int",
					"name": "string"
				}
			}
		}
	]
}
```

## POST /create/team

Creates a new team. The request's body needs too have a `JSON` `BODY` with the following format:

```json
{
    "name": "string*",
    "description": "string*",
	"creation_date": "string (YYYY-MM-DD)*",
    "location": "string*",
    "logo_url": "string?",
    "banner_url": "string?"
}
```

This method returns the team's `id`.

## POST /add/label

Adds a new label to a team. The request's body needs too have a `JSON` `BODY` with the following format:

```json
{
    "team_id": "int*",
    "label_id": "int*"
}
```

## POST /add/badge

Adds a new badge to a team. The request's body needs too have a `JSON` `BODY` with the following format:

```json
{
    "team_id": "int*",
    "badge_id": "int*",
    "acquisition_date": "string (YYYY-MM-DD)*"
}
```

## POST /add/person

Add a new person to a team. The request's body needs to have a `JSON` `BODY` with the following format:

```json
{
    "team_id": "string*",
    "name": "string*",
    "career": "string*",
    "graduation_date": "string (YYYY-MM-DD)*",
    "picture_url": "string?",
    "portafolio_url": "string?"
}
```

## POST /create/badge

Creates a new badge. The request's body needs to have a `JSON` `BODY` with the following format:

```json
{
    "name": "string*",
    "description": "string*",
    "points": "int*",
    "category": "int*"
}
```

This method returns the badge's `id`.

## POST /create/label

Creates a new label. The request's body needs to have a `JSON` `BODY` with the following format:

```json
{
	"name": "string*"
}
```

This method returns the labels's `id`.

## POST /create/category

Creates a new category. The request's body needs to have a `JSON` `BODY` with the following format:

```json
{
    "name": "string*"
}
```

This method returns the category's `id`.

## POST /delete

| PARAM | Content         |
|-------|-----------------|
| kind* | type of object  |
| id*   | id of object    |
| force | (false default) |

`kind` $\in$ { `label`, `badge`, `person`, `category`, `team` }

`force` can be `true` or `false`. If it is `true`, any object with a link to that object will also be deleted.

## POST /edit

| PARAM | Content        |
|-------|----------------|
| kind* | type of object |
| id*   | id of object   |

If any of the values in the `JSON`s are not set in them, they will be kept as is. If you wish to set a nullable value to `null`, pass in `"null"` as the value.

### kind == label

```json
{
	"name": "string?",
}
```

### kind == badge

```json
{
	"name": "string?",
	"description": "string?",
	"points": "int?",
	"category": "int?", 
}
```


### kind == person

```json
{
	"team_id": "int?",
	"name": "string",
	"career": "string?",
	"graduation_date": "string (YYYY-MM-DD)?",
	"picture_url": "string?",
	"portafolio_url": "stirng?",
}
```

### kind == team

```json
{
	"name": "string",
	"description": "string?",
	"stage": "int?",
	"creation_date": "string (YYYY-MM-DD)?",
	"logo_url": "string?",
	"banner_url": "stirng?",
	"location": "location",
}
```

In return you will get a list of values that where successfully added and the ones that failed.

# POST /delete\_ownership

Delete a link between between a team and a label or badge

| PARAM | Content        |
|-------|----------------|
| kind* | type of object |

If `kind` is `label` send a `JSON` `BODY`:
```json
{
    "team_id": "int",
    "label_id": "int"
}
```

If `kind` is `badge` send a `JSON` `BODY`:
```json
{
    "team_id": "int",
    "badge_id": "int"
}
```

# POST /update/rankings

Update scores and rankings.

## TODO

- loop to bind in edit
