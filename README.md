# API Documentation

## GET /teams

Returns a list of teams.

```json
[
	{
		"id": int,
		"rank": int,
		"score": int,
		"stage": int,
		"name": string,
		"logo_url": string,
		"banner_url": string,
		"description": string,
		"creation_date": string,
		"location": string,
		"labels":
		[
			{
				"id": int,
				"name": string
			}
		],
		"persons":
		[
			{
				"id": int,
				"team_id": int,
				"name": string,
				"career": string,
				"graduation_date": string,
				"picture_url": string,
				"portafolio_url": string
			}
		]
		"badges":
		[
			{
				"id": int,
				"acquisition_date": string,
				"badge": {
					"id": int,
					"name": string,
					"description": string,
					"points": int,
					"category": {
						"id": int,
						"name": string
					}
				}
			}
		]
	}
]
```

- `rank` is a nullable value
- `logo_url` is a nullable value
- `banner_url` is a nullable value
- `creation_date` has format YYYY-MM-DD
- `picture_url` is a nullable value
- `portafolio_url` is a nullable value
- `graduation_date` has format YYYY-MM-DD
- `acquisition_date` has format YYYY-MM-DD

| Status Code | Meaning  |
|-------------|----------|
| 202         | ACCEPTED |
| 404         | ERROR    |
|             |          |

## GET /badges

Return all badges 

```json
[
	"badge": {
		"id": int,
		"name": string,
		"description": string,
		"points": int,
		"category": {
			"id": int,
			"name": string
		}
	}
]
```

## GET /labels

Return all labels

```json
[
	{
		"id": int,
		"name": string
	}
]
```

## GET /categories

Return all categories

```json
[
	{
		"id": int,
		"name": string
	}
]
```

## GET /team/{id}

Return a team with `id`

```json
{
	"id": int,
	"rank": int,
	"score": int,
	"stage": int,
	"name": string,
	"logo_url": string,
	"banner_url": string,
	"description": string,
	"creation_date": string,
	"location": string,
	"labels":
	[
		{
			"id": int,
			"name": string
		}
	],
	"persons":
	[
		{
			"id": int,
			"team_id": int,
			"name": string,
			"career": string,
			"graduation_date": string,
			"picture_url": string,
			"portafolio_url": string
		}
	]
	"badges":
	[
		{
			"id": int,
			"acquisition_date": string,
			"badge": {
				"id": int,
				"name": string,
				"description": string,
				"points": int,
				"category": {
					"id": int,
					"name": string
				}
			}
		}
	]
}
```

- `rank` is a nullable value
- `logo_url` is a nullable value
- `banner_url` is a nullable value
- `creation_date` has format YYYY-MM-DD
- `picture_url` is a nullable value
- `portafolio_url` is a nullable value
- `graduation_date` has format YYYY-MM-DD
- `acquisition_date` has format YYYY-MM-DD

## POST /create/team

Create a new team. The request's body needs too have a json `BODY` with the following format:
```json
{
    "name": string,
    "description": string,
	"creation_date": string,
    "location": string,
}
```

- `creation_date` should have format YYYY-MM-DD

This method returns a redirect to `team/id`, corresponding to the team's id.

## POST /add/label

Add a new label to a team. The request's body needs too have a json `BODY` with the following format:

```json
{
    "team_id": int,
    "label_id": int
}
```

## POST /add/badge

Add a new badge to a team. The request's body needs too have a json `BODY` with the following format:

```json
{
    "team_id": int,
    "badge_id": int,
    "acquisition_date": string
}
```

- `acquisition_date` should have format  YYYY-MM-DD

## POST /add/person

Add a new person to a team. The request's body needs to have a json `BODY` with the following format:

```json
{
    "team_id": string,
    "name": string,
    "career": string,
    "graduation_date": string
}
```

- `graduation_date` should have format  YYYY-MM-DD

## POST /create/badge

Create a new badge. The request's body needs to have a json `BODY` with the following format:

```json
{
    "name": string,
    "description": string,
    "points": int,
    "category": int
}
```

## POST /create/label

Create a new label. The request's body needs to have a json `BODY` with the following format:

```json
{
	"name": string,
}
```

## POST /create/category

Create a new category. The request's body needs to have a json `BODY` with the following format:

```json
{
    "name": string
}
```
