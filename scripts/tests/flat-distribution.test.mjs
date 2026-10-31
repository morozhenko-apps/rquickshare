import test from "node:test";
import assert from "node:assert/strict";
import {existsSync, readFileSync} from "node:fs";
import {validateR2Receipt, deletePublishedArtifacts} from "../cleanup-published-artifacts.mjs";

const BASE="https://download.morozhenko.com/dist/rQuickShare/";
const HASH="a".repeat(64), COMMIT="b".repeat(40);
const proof=url=>({url,sha256:HASH,readback_sha256:HASH,size:123,readback_size:123,readback_status:200});
const receipt=()=>({schema:1,product:"rQuickShare",version:"0.9.0",revision:1,commit:COMMIT,base_url:BASE,
  latest:proof(BASE+"latest.json"),artifacts:[{name:"native-package",run_id:51,files:[
    proof(BASE+"linux/rquickshare-native_0.9.0-1_amd64.deb"),
    proof(BASE+"linux/rquickshare-native_0.9.0-1_amd64.deb.sha256")]}]});
const read=file=>readFileSync(new URL("../../"+file,import.meta.url),"utf8").replace(/\r\n/g,"\n");

test("Given_a_flat_release_When_workflow_is_inspected_Then_tag_signing_readback_and_cleanup_gates_remain",()=>{
  const workflow=read(".github/workflows/native-tag-publish.yml");
  assert.match(workflow,/tags: \["native\/v\*"\]/);
  assert.ok(workflow.includes('git fetch --no-tags origin "refs/tags/$GITHUB_REF_NAME:refs/release-validation/native-tag"'));
  assert.ok(workflow.includes('test "$(git cat-file -t refs/release-validation/native-tag)" = tag'));
  assert.ok(workflow.includes('"$(git rev-parse refs/release-validation/native-tag^{commit})" == "$GITHUB_SHA"'));
  assert.ok(workflow.includes('"$(git rev-parse HEAD)" == "$(git rev-parse origin/master^{commit})"'));
  assert.match(workflow,/git rev-parse origin\/master\^\{commit\}/);
  assert.match(workflow,/environment: apt-release/);
  assert.ok(!workflow.includes('uses: ./.github/workflows/mode-b.yml'), 'release must not invoke pre-release Mode B');
  assert.match(workflow,/--receipt "\$RUNNER_TEMP\/publication-receipt.json"/);
  assert.match(workflow,/needs: \[admission, publish, cleanup\]/);
  assert.match(workflow,/scripts\/cleanup-published-artifacts.mjs/);
  assert.match(workflow,/retention-days: 1/);
  assert.equal((workflow.match(/^\s+(?:-\s+)?uses:\s+actions\/upload-artifact@/gm)||[]).length,1);
  assert.doesNotMatch(workflow,/name: unsigned-apt/);
  assert.doesNotMatch(workflow,/infra\/native-download|wrangler|pool\/main|dists\/|gh release upload|apt-enrollment/);
  assert.equal(existsSync(new URL("../../infra/native-download",import.meta.url)),false);
  assert.equal(existsSync(new URL("../../.github/workflows/native-download-check.yml",import.meta.url)),false);
  const refresh=read(".github/workflows/native-apt-refresh.yml");
  assert.match(refresh,/rqs-previous\/linux\/\*\.deb/);
  assert.match(refresh,/dist\/rQuickShare\/linux\/InRelease/);
  assert.match(refresh,/--commit "\$\(git rev-parse HEAD\)" --run-id "\$GITHUB_RUN_ID"/);
  assert.doesNotMatch(refresh,/snapshots\/|pool\/|dists\//);
});

test("Given_malformed_receipts_When_cleanup_is_requested_Then_no_GitHub_request_is_made",async()=>{
  validateR2Receipt(receipt());
  const changes=[
    ["missing pointer",r=>delete r.latest],
    ["bad latest hash",r=>r.latest.readback_sha256="f".repeat(64)],
    ["bad latest HTTP",r=>r.latest.readback_status=404],
    ["wrong product",r=>r.product="RepoTicker"],
    ["wrong origin",r=>r.base_url="https://other.invalid/"],
    ["bad commit",r=>r.commit="bad"],
    ["zero revision",r=>r.revision=0],
    ["overflow revision",r=>r.revision=2147483648],
    ["missing sidecar",r=>r.artifacts[0].files.pop()],
    ["duplicate entry",r=>r.artifacts[0].files[1]=r.artifacts[0].files[0]],
    ["foreign artifact",r=>r.artifacts[0].name="other-package"],
    ["foreign file",r=>r.artifacts[0].files[0].url=BASE+"windows/setup.exe"],
    ["bad readback",r=>r.artifacts[0].files[0].readback_size--],
    ["bad hash",r=>r.artifacts[0].files[0].readback_sha256="f".repeat(64)],
    ["zero run",r=>r.artifacts[0].run_id=0],
    ...["?query","#fragment",".zip"].map(suffix=>[suffix,r=>r.artifacts[0].files[0].url+=suffix]),
    ["traversal",r=>r.artifacts[0].files[0].url=r.artifacts[0].files[0].url.replace("/linux/","/linux/../")],
    ["foreign version",r=>r.artifacts[0].files[0].url=r.artifacts[0].files[0].url.replace("0.9.0-1","0.9.1-1")],
  ];
  for(const [label,change] of changes){
    const bad=receipt();change(bad);let calls=0;
    await assert.rejects(deletePublishedArtifacts(bad,"fixture",async()=>{calls++;throw Error("network forbidden");}),undefined,label);
    assert.equal(calls,0,label);
  }
});

test("Given_verified_receipt_When_cleanup_runs_Then_only_exact_source_archive_is_deleted_and_confirmed",async()=>{
  let artifacts=[{id:101,name:"native-package",workflow_run:{id:51,head_sha:COMMIT}},
    {id:102,name:"other-build",workflow_run:{id:51,head_sha:COMMIT}}];
  const deleted=[];
  const request=async(url,options={})=>{
    assert.ok(url.startsWith("https://api.github.com/repos/morozhenko-apps/rquickshare/"));
    if(options.method==="DELETE"){
      const id=Number(url.match(/\/artifacts\/(\d+)$/)[1]);deleted.push(id);
      artifacts=artifacts.filter(item=>item.id!==id);return {status:204};
    }
    return {status:200,json:async()=>({artifacts})};
  };
  assert.equal(await deletePublishedArtifacts(receipt(),"fixture",request),1);
  assert.deepEqual(deleted,[101]);
  assert.equal(artifacts[0].name,"other-build");
  assert.equal(await deletePublishedArtifacts(receipt(),"fixture",request),0);
});

test("Given_foreign_ambiguous_or_failed_GitHub_state_When_cleanup_runs_Then_it_fails_closed",async()=>{
  const original={id:101,name:"native-package",workflow_run:{id:51,head_sha:COMMIT}};
  for(const [label,entries,status] of [
    ["foreign commit",[{...original,workflow_run:{id:51,head_sha:"f".repeat(40)}}],200],
    ["foreign run",[{...original,workflow_run:{id:52,head_sha:COMMIT}}],200],
    ["duplicate",[original,{...original,id:102}],200],
    ["invalid id",[{...original,id:0}],200],
    ["forbidden listing",[],403],
    ["malformed listing",null,200],
  ]){
    let deletes=0;
    await assert.rejects(deletePublishedArtifacts(receipt(),"fixture",async(_url,options={})=>{
      if(options.method==="DELETE")deletes++;
      return {status,json:async()=>({artifacts:entries})};
    }),undefined,label);
    assert.equal(deletes,0,label);
  }
  for(const mode of ["delete-fails","still-listed"]){
    await assert.rejects(deletePublishedArtifacts(receipt(),"fixture",async(_url,options={})=>{
      if(options.method==="DELETE")return {status:mode==="delete-fails"?503:204};
      return {status:200,json:async()=>({artifacts:[original]})};
    }),mode==="delete-fails"?/cleanup failed/:/remains/);
  }
  let calls=0;
  await assert.rejects(deletePublishedArtifacts(receipt(),"",async()=>{calls++;}),/Missing Actions-write/);
  assert.equal(calls,0);
});
