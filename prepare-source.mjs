import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {mkdirSync,readFileSync,writeFileSync} from 'node:fs';
const run=(...args)=>execFileSync(process.env.PAINT_QUERY_CLI||'calcit',args,{encoding:'utf8',maxBuffer:30*1024*1024,stdio:['ignore','pipe','ignore']});
const q=(...args)=>JSON.parse(run('query',...args,'--format','json')).data;
mkdirSync('acceptance',{recursive:true});
function inventory(){
  const defs=[];
  for(const ns of q('ns').namespaces){
    const names=run('query','defs',ns).split('\n').filter(l=>/^  \S/.test(l)).map(l=>l.trim().split(' ')[0]);
    assert.equal(names.length,q('ns',ns).definition_count);
    for(const name of names)defs.push(q('def',`${ns}/${name}`));
  }
  return defs;
}
if(process.argv[2]==='inventory'){
  const defs=inventory();writeFileSync('acceptance/reviewed.json',JSON.stringify(defs,null,2));
  console.log(JSON.stringify({definitions:defs.length,tests:defs.reduce((n,d)=>n+d.tests.length,0),namespaces:q('ns').namespaces}));process.exit(0);
}
assert.equal(run('-v').trim(),'0.30.1');run('docs','agents','--full');
const before=inventory(),config=q('config');assert.deepEqual(before,JSON.parse(readFileSync('acceptance/reviewed.json','utf8')));
const expected=structuredClone(before),matches=q('search','^(merge|count)$','--regex','--source','project');
assert.equal(matches.summary.matches,15);assert.equal(matches.summary.definitions,5);
const ops=[],counts={merge:0,count:0};
for(const d of matches.definitions){
  assert.ok(['calcit-paint.ui/button','calcit-paint.ui/touch-container','calcit-paint.ui/text-input-focus-area','calcit-paint.main/handle-paint-event!','calcit-paint.main/validate-scene-demo!'].includes(d.id));
  const definition=expected.find(x=>x.id===d.id);
  for(const m of d.matches){
    assert.equal(m.node_kind,'call');assert.ok(['merge','count'].includes(m.tree));
    const path=m.path.replace(/^code@/,'@'),indices=path.slice(1).split('.').map(Number);
    let node=definition.code;for(const i of indices.slice(0,-1))node=node[i];
    const i=indices.at(-1);assert.equal(node[i],m.tree);
    const replacement=m.tree==='merge'?'.merge':'.len';node[i]=replacement;counts[m.tree]++;
    ops.push(['tree','replace',d.id,'--path',path,'--input-format','json-ast','--code',JSON.stringify(replacement),'--expect',`quote ${m.tree}`]);
  }
}
assert.deepEqual(counts,{merge:10,count:5});
const structured=expected.find(d=>d.id==='calcit-paint.core/validate-scene-structured');
let exampleChanges=0;
function updateExample(n){
  if(!Array.isArray(n))return n;
  const result=n.map(updateExample);
  if(result[0]==='count'){assert.deepEqual(result,['count','diagnostics']);result[0]='.len';exampleChanges++;}
  return result;
}
structured.examples=structured.examples.map(updateExample);assert.equal(exampleChanges,1);
const examplesCode=run('cirru','format',JSON.stringify(structured.examples.map(example=>['quote',example])));
ops.push(['edit','examples',structured.id,'--code',examplesCode]);
writeFileSync('acceptance/before.json',JSON.stringify(before,null,2));writeFileSync('acceptance/operations.json',JSON.stringify(ops,null,2));
const args=['edit','transaction','--file','acceptance/operations.json','--format','json'];
const preview=JSON.parse(run(...args,'--dry-run'));assert.ok(preview.scoped_revision);
writeFileSync('acceptance/preview.json',JSON.stringify(preview,null,2));writeFileSync('acceptance/applied.json',run(...args,'--expect-revision',preview.scoped_revision));
assert.deepEqual(inventory(),expected,'Only15callheads; all schemas/FFI/callbacks/fields/tests/assertions/docs/metadata unchanged');assert.deepEqual(q('config'),config);
writeFileSync('acceptance/after.json',JSON.stringify(expected,null,2));writeFileSync('acceptance/snapshot-blob.txt',execFileSync('git',['hash-object','calcit.cirru'],{encoding:'utf8'}));
console.log(JSON.stringify({definitions:expected.length,counts,exampleChanges,changedDefinitions:6,tests:expected.reduce((n,d)=>n+d.tests.length,0)}));
