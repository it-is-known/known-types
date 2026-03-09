# This is free and unencumbered software released into the public domain.

module OpenAPI; end
module OpenAPI::Transforms; end

##
# Removes redundant `oneOf`, `anyOf`, and `allOf` wrappers.
module OpenAPI::Transforms::RemoveRedundantCombines
  ##
  # @param  [Hash] The flattened `openapi.components.schemas` map
  # @return [void]
  def self.transform_schemas!(schemas)
    raise ArgumentError, schemas.inspect unless schemas.is_a?(Hash)
    schemas.each_value do |schema|
      transform_schema!(schema, schemas)
    end
  end # transform_schemas!

  protected

  ##
  # @param  [Hash] A JSON Schema schema type
  # @param  [Hash] The flattened `openapi.components.schemas` map
  # @return [void]
  def self.transform_schema!(schema, schemas)
    raise ArgumentError, schema.inspect unless schema.is_a?(Hash)
    raise ArgumentError, schemas.inspect unless schemas.is_a?(Hash)
    case
      when schema[:oneOf], schema[:anyOf], schema[:allOf]
        variants = schema[:oneOf] || schema[:anyOf] || schema[:allOf] || []
        raise ArgumentError, schema.inspect unless variants.all? { it.is_a?(Hash) }
        variants.reject! { it[:'$recursiveRef'] }
        if variants.size == 1
          schema.merge!(variants.first)
          %i(oneOf anyOf allOf).each { schema.delete(it) }
        elsif variants.all? { it[:required] }
          variants.each { it.delete(:required) }
          variants.reject! { it.empty? }
          %i(oneOf anyOf allOf).each { schema.delete(it) } if variants.empty?
        else
          variants.each do |variant|
            transform_schema!(variant, schemas)
          end
        end
      else # nothing to do
    end
  end # transform_schema!
end # OpenAPI::Transforms::RemoveRedundantCombines
