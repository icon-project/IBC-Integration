package icon

type Query struct {
	MethodName string
	Value      Value
}

type Value struct {
	Params map[string]interface{} `json:"params,omitempty"`
}
