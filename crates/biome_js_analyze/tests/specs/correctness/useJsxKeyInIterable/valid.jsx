/* should not generate diagnostics */

import React from "react";

[<Hello key="first" />, <Hello key="second" />, <Hello key="third" />];

[...[<Hello key="first" />, <Hello key="second" />], <Hello key="third" />];

[<Hello key="first" />, xyz ? <Hello key="second"/>: <Hello key="second" />, <Hello key="third" />];

[<React.Fragment key="first"></React.Fragment>, <React.Fragment key="second"></React.Fragment>, <React.Fragment key="third"></React.Fragment>];

data.map(x => <Hello key={x}>{x}</Hello>);

data.map(x => <React.Fragment key={x}>{x}</React.Fragment>);

data.forEach(x => data1.push(<React.Fragment key={x}>{x}</React.Fragment>));

Array.from([1, 2, 3], (x) => <Hello key={x}>{x}</Hello>);

Array.from([1, 2, 3], (x) => {
	return <Hello key={x}>{x}</Hello>
});

[React.createElement("h1", {key}), React.createElement("h1", {key: "second"}), React.createElement("h1", {key: third})];

data.map(c => React.createElement("h1", {key}));

React.Children.map(c => React.cloneElement(c, {key: c}));

(<h1></h1>)

(<h1>
		<h2></h2>
		<h2></h2>
		<h2></h2>
		<h2></h2>
</h1>)

(<h1>{data.map(x => <h1 key={x}>{x}</h1>)}</h1>)

(<h1>{[<h1 key={1}></h1>, <h1 key={2}></h1>, <h1 key={3}></h1>]}</h1>)

(<h1>{data.map(c => <h1 key={c}></h1>)}</h1>)

(<h1>{data.map(c => (<h1 key={c}></h1>))}</h1>)

(<h1>{data.map(c => {return (<h1 key={c}></h1>)})}</h1>)

<>{data.reduce((total, next) => total + next, 0)}</>

<>{data.reduce((a, b) => Math.max(a, b), 0)}</>

<>{data.reduce((a, b) => a > b ? a : b, 0)}</>

[].map((item) => {
	return item.condition ? <div key={item.id} /> : <div key={item.id}>foo</div>;
});

[].map(function(item) {
	const x = 5;
	return <div key={item.id}>{x}</div>;
});

[].map(function(item) {
	const x = 5;
	const div = <div key={item.id}>{x}</div>;
	return div;
});

[].map((item) => {
	const node = <button type="button">{item.label}</button>;
	return <Fragment key={item.label}>{node}</Fragment>;
});

[].map((el) => {
  const content = <p>Paragraph</p>
  return (<div key={el}>{content}</div>);
});

// is only checked when options.checkShorthandFragment is set
data.map((x) => <>x</>)

data.map((x) => {
	if (x.type === 'string') {
		return <div key={x.value}>{x.value}</div>
	}
	return <div key={x.value}>{x.value}</div>
})

data.map((x) => {
	switch (x.type) {
		case 'string':
			return <div key={x.value}>{x.value}</div>
		case 'number':
			return <div key={x.value}>{x.value}</div>
		default:
			return <div key={x.value}>{x.value}</div>
	}
})

const Valid = [<>
	<p>Test 1</p>
	<p>Test 2</p>
</>]

[<>
	<p>Test 1</p>
	<p>Test 2</p>
	<p>Test 3</p>
</>]