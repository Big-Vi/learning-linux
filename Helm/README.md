Helm is a package manager for K8S. Helm chart installs all the objects/resources required in a single command.  

## Helm components
- Chart
- Repository
- Release
- Revision
- Stores all its metadata as a Secret

`helm install <release-name> <chart-name>`

Helm hub = ArtifactHub.io

## Helm basic
```
helm --help
helm repo add <chart>
helm search repo <repo-name>
helm search hub <hub-name>
```

## Customize chart parameter

To override custom parameters:  
1 - Set command line argument.  

```
helm install --set <param-name>=<param-value> <release> <chart>
helm install --values custom-values.yml <release> <chart>
```
2 - Edit the values.yml of the Helm repo by pulling it to a local directory.  

```
helm pull <chart>
helm pull --untar <chart>
helm install <release-name> <local-dir>
```

## Lifecycle Management

```
helm upgrade <release-name> <chart-name>
helm rollback <release-name> <revision-number>
```

## Writing Helm Chart

`helm create <chart-name>` - Creates skeleton  

```
apiVersion: apps/v1
kind: Deployment
metadata:
	name: {{ .Release.Name }}-nginx
spec:
	replicas: {{ .Values.replicaCount }}
```

> values.yml
```
replicaCount: 2
```

## Validating Chart

```
helm lint <dir-name>
helm template <release-name>(optional) <dir-name>
helm template <dir-name> --debug

helm install <release-name> <dir-name> --dry-run
```

## Functions & Pipelines

upper, quote, replace, shuffle, contains, etc...  
E.g.,
```
apiVersion: apps/v1
kind: Deployment
spec:
	image: {{ default "nginx" .Values.image.repository | lower }}
```

## Conditionals
E.g.,
```
apiVersion: apps/v1
kind: Deployment
metadata:
	{{- if .Values.backEnd }}
		labels:
			app: {{ .Values.backEnd }}
	{{- else if eq .Values.backEmd "frontend" }}
		labels:
			app: Front end
	{{- end }}
```

add `-` to trim the white space in the output yml file.  

## With Blocks
E.g.,
```
apiVersion: apps/v1
kind: Deployment
metadata:
	{{- with .Values }}
		{{- if .backEnd }}
			labels:
				app: {{ .backEnd }}
		{{- else if eq .backEmd "frontend" }}
			labels:
				app: Front end
		{{- end }}
	{{- end }}
```
The root is represented with $.  
.Release.Name = $.Release.Name  

## Ranges

> values.yml
```
regions:
	- ap-southeast-1
	- ap-southeast-2
	- mumbai
```

```
apiVersion: v1
kind: ConfigMap
data:
regions:
	{{- range .Values.regions}}
	- {{ . }}
	{{- end}}
```

## Named Templates

> _helper.tpl
```
{{- define "labels" }}
	app: frontend
	tier: web
{{- end }}
```

```
apiVersion: apps/v1
kind: Deployment
metadata:
	labels:
		{{- template "labels" . }}
```

  

Replace the template with include to solve the space issue.  
```
apiVersion: apps/v1
kind: Deployment
metadata:
	labels:
		{{- include "labels" . | indent 2 }}
```

## Chart Hooks

Use case: Backing up DB before upgrading the chart.  

helm upgrade -> verify -> render -> pre-upgrade -> upgrade -> post-upgrade  
helm install -> verify -> render -> pre-install -> install -> post-install  

DB can be backed up in a pre-upgrade hook.  
Email can be sent on the post-upgrade hook.  
