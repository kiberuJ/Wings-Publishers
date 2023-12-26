export const idlFactory = ({ IDL }) => {
  const ArticlePayload = IDL.Record({
    'title' : IDL.Text,
    'content' : IDL.Text,
    'writer_id' : IDL.Nat64,
  });
  const Article = IDL.Record({
    'id' : IDL.Nat64,
    'title' : IDL.Text,
    'updated_at' : IDL.Opt(IDL.Nat64),
    'content' : IDL.Text,
    'reviewer_id' : IDL.Opt(IDL.Nat64),
    'writer_id' : IDL.Nat64,
    'created_at' : IDL.Nat64,
    'state' : IDL.Text,
  });
  const Error = IDL.Variant({
    'NotFound' : IDL.Record({ 'msg' : IDL.Text }),
    'Exists' : IDL.Record({ 'msg' : IDL.Text }),
    'ServerError' : IDL.Record({ 'msg' : IDL.Text }),
  });
  const Result = IDL.Variant({ 'Ok' : Article, 'Err' : Error });
  const UserPayload = IDL.Record({ 'name' : IDL.Text, 'role' : IDL.Text });
  const User = IDL.Record({
    'id' : IDL.Nat64,
    'updated_at' : IDL.Opt(IDL.Nat64),
    'name' : IDL.Text,
    'role' : IDL.Text,
    'created_at' : IDL.Nat64,
  });
  const Result_1 = IDL.Variant({ 'Ok' : User, 'Err' : Error });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : Error });
  return IDL.Service({
    'add_article' : IDL.Func([ArticlePayload], [Result], []),
    'add_user' : IDL.Func([UserPayload], [Result_1], []),
    'approve' : IDL.Func([IDL.Nat64, IDL.Nat64], [Result], []),
    'delete_user' : IDL.Func([IDL.Nat64], [Result_1], []),
    'get_article' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'get_article_summary' : IDL.Func([IDL.Nat64], [Result_2], ['query']),
    'get_user' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'request_review' : IDL.Func([IDL.Nat64, IDL.Nat64], [Result], []),
    'update_user' : IDL.Func([IDL.Nat64, UserPayload], [Result_1], []),
  });
};
export const init = ({ IDL }) => { return []; };
