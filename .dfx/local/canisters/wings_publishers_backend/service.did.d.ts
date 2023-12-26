import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Article {
  'id' : bigint,
  'title' : string,
  'updated_at' : [] | [bigint],
  'content' : string,
  'reviewer_id' : [] | [bigint],
  'writer_id' : bigint,
  'created_at' : bigint,
  'state' : string,
}
export interface ArticlePayload {
  'title' : string,
  'content' : string,
  'writer_id' : bigint,
}
export type Error = { 'NotFound' : { 'msg' : string } } |
  { 'Exists' : { 'msg' : string } } |
  { 'ServerError' : { 'msg' : string } };
export type Result = { 'Ok' : Article } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : User } |
  { 'Err' : Error };
export type Result_2 = { 'Ok' : string } |
  { 'Err' : Error };
export interface User {
  'id' : bigint,
  'updated_at' : [] | [bigint],
  'name' : string,
  'role' : string,
  'created_at' : bigint,
}
export interface UserPayload { 'name' : string, 'role' : string }
export interface _SERVICE {
  'add_article' : ActorMethod<[ArticlePayload], Result>,
  'add_user' : ActorMethod<[UserPayload], Result_1>,
  'approve' : ActorMethod<[bigint, bigint], Result>,
  'delete_user' : ActorMethod<[bigint], Result_1>,
  'get_article' : ActorMethod<[bigint], Result>,
  'get_article_summary' : ActorMethod<[bigint], Result_2>,
  'get_user' : ActorMethod<[bigint], Result_1>,
  'request_review' : ActorMethod<[bigint, bigint], Result>,
  'update_user' : ActorMethod<[bigint, UserPayload], Result_1>,
}
